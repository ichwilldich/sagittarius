use std::str::FromStr;

use axum_extra::headers::authorization::Credentials;
use http::HeaderValue;
use ichwilldich_lib::{bail, error::ErrorReport};

pub struct AWS4 {
  pub credential: AWS4Credential,
  pub signed_headers: Vec<String>,
  pub signature: String,
}

#[derive(Debug)]
pub struct AWS4Credential {
  pub access_key: String,
  pub date: String,
  pub region: String,
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
          credential = Some(parts[1].parse().ok()?);
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

impl FromStr for AWS4Credential {
  type Err = ErrorReport;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split('/').collect::<Vec<_>>();
    if parts.len() != 5 || parts[3] != "s3" || parts[4] != "aws4_request" {
      bail!("Invalid AWS4 credential format {s}");
    }

    Ok(AWS4Credential {
      access_key: parts[0].to_string(),
      date: parts[1].to_string(),
      region: parts[2].to_string(),
    })
  }
}
