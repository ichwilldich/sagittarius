use std::time::SystemTime;

use axum::{
  RequestPartsExt,
  body::Bytes,
  extract::{FromRequest, Request},
};
use axum_extra::{
  TypedHeader,
  headers::{Authorization, Date, authorization::Credentials},
};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use http::HeaderValue;
use ichwilldich_lib::error::{Error, Result};
use sha2::Sha256;

use crate::s3::{
  auth::to_sign::{canonical_request, string_to_sign},
  header::AwzDate,
};

mod to_sign;

/// TODO: check date and region from request, query sort, load the correct secret, body handling
/// https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html
pub struct S3Auth {
  region: String,
  access_key: String,
}

const SECRET: &str = "secret";

impl<S: Sync + Send> FromRequest<S> for S3Auth {
  type Rejection = Error;

  async fn from_request(req: Request, state: &S) -> std::result::Result<Self, Self::Rejection> {
    let (mut req, body) = req.into_parts();
    let TypedHeader(Authorization(mut auth)) =
      req.extract::<TypedHeader<Authorization<AWS4>>>().await?;

    let date = {
      if let Ok(TypedHeader(date)) = req.extract::<TypedHeader<AwzDate>>().await {
        DateTime::<Utc>::from_naive_utc_and_offset(date.into_inner(), Utc)
      } else if let Ok(TypedHeader(date)) = req.extract::<TypedHeader<Date>>().await {
        DateTime::<Utc>::from(SystemTime::from(date))
      } else {
        Utc::now()
      }
    };

    for header in req.headers.keys() {
      if header.as_str().starts_with("x-amz-")
        && !auth
          .signed_headers
          .iter()
          .any(|h| h.eq_ignore_ascii_case(header.as_str()))
      {
        return Err(Error::BadRequest);
      }
    }

    let body = Bytes::from_request(Request::from_parts(req.clone(), body), state).await?;
    let canonical_request =
      canonical_request(&req, &mut auth, &to_sign::Payload::SingleChunk(&body));
    let to_sign = string_to_sign(&canonical_request, &date, &auth.credential);

    let date_key = str_hmac(format!("AWS4{}", SECRET).as_bytes(), &auth.credential.date)?;
    let date_region_key = str_hmac(&date_key, &auth.credential.region)?;
    let date_region_service_key = str_hmac(&date_region_key, "s3")?;
    let signing_key = str_hmac(&date_region_service_key, "aws4_request")?;
    let signature = str_hmac(&signing_key, &to_sign)?;

    let signature = hex::encode(signature);
    if signature != auth.signature {
      return Err(Error::Forbidden);
    }

    Ok(S3Auth {
      region: auth.credential.region,
      access_key: auth.credential.access_key,
    })
  }
}

fn str_hmac(key: &[u8], msg: &str) -> Result<Vec<u8>> {
  let mut hmac = Hmac::<Sha256>::new_from_slice(key)?;
  hmac.update(msg.as_bytes());
  Ok(hmac.finalize().into_bytes().to_vec())
}

struct AWS4 {
  credential: AWS4Credential,
  signed_headers: Vec<String>,
  signature: String,
}

struct AWS4Credential {
  access_key: String,
  date: String,
  region: String,
}

impl Credentials for AWS4 {
  const SCHEME: &'static str = "AWS4-HMAC-SHA256";

  fn decode(value: &HeaderValue) -> Option<Self> {
    debug_assert!(
      value.as_bytes()[..Self::SCHEME.len()].eq_ignore_ascii_case(Self::SCHEME.as_bytes()),
      "HeaderValue to decode should start with \"AWS4-HMAC-SHA256 ..\", received = {:?}",
      value,
    );

    let bytes = &value.as_bytes()["AWS4-HMAC-SHA256 ".len()..];
    let str = std::str::from_utf8(bytes).ok()?;

    let parts = str.split(',').collect::<Vec<_>>();
    if parts.len() != 3 {
      return None;
    }

    let mut credential = None;
    let mut signed_headers = None;
    let mut signature = None;
    for part in parts {
      let parts = part.trim().split('=').collect::<Vec<_>>();
      if parts.len() != 2 {
        return None;
      }

      match parts[0] {
        "Credential" => {
          let parts = parts[1].split('/').collect::<Vec<_>>();
          if parts.len() != 5 {
            return None;
          }

          if parts[3] != "s3" || parts[4] != "aws4_request" {
            return None;
          }

          credential = Some(AWS4Credential {
            access_key: parts[0].to_string(),
            date: parts[1].to_string(),
            region: parts[2].to_string(),
          });
        }
        "SignedHeaders" => {
          let mut headers = parts[1]
            .split(';')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
          if !headers.iter().any(|s| s.eq_ignore_ascii_case("host")) {
            return None;
          }
          headers.sort_unstable_by_key(|s| s.to_ascii_lowercase());

          signed_headers = Some(headers)
        }
        "Signature" => signature = Some(parts[1].to_string()),
        _ => return None,
      }
    }

    if credential.is_none() || signed_headers.is_none() || signature.is_none() {
      return None;
    }

    Some(AWS4 {
      credential: credential.unwrap(),
      signed_headers: signed_headers.unwrap(),
      signature: signature.unwrap(),
    })
  }

  fn encode(&self) -> HeaderValue {
    unimplemented!()
  }
}
