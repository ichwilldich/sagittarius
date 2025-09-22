use std::time::SystemTime;

use axum::{
  RequestPartsExt,
  body::Bytes,
  extract::{FromRequest, Request},
};
use axum_extra::{
  TypedHeader,
  headers::{Authorization, Date},
};
use chrono::{DateTime, Utc};
use http::request::Parts;
use ichwilldich_lib::error::{Error, Result};

use crate::s3::{
  auth::{
    S3Auth, SECRET,
    credential::AWS4,
    sig_v4::{CanonicalRequest, Payload},
  },
  header::AwzDate,
};

pub async fn header_auth(req: Request) -> Result<S3Auth> {
  let (mut parts, body) = req.into_parts();
  let TypedHeader(Authorization(mut auth)) =
    parts.extract::<TypedHeader<Authorization<AWS4>>>().await?;

  let date = {
    if let Ok(TypedHeader(date)) = parts.extract::<TypedHeader<AwzDate>>().await {
      DateTime::<Utc>::from_naive_utc_and_offset(date.into_inner(), Utc)
    } else if let Ok(TypedHeader(date)) = parts.extract::<TypedHeader<Date>>().await {
      DateTime::<Utc>::from(SystemTime::from(date))
    } else {
      Utc::now()
    }
  };
  check_headers(&parts, &auth)?;

  let body = Bytes::from_request(Request::from_parts(parts.clone(), body), &()).await?;

  let signature = CanonicalRequest::new(&parts, &mut auth, &Payload::SingleChunk(&body))
    .string_to_sign(&date, &auth.credential)
    .sign(SECRET, &auth.credential)?;

  if signature != auth.signature {
    return Err(Error::Forbidden);
  }
  Ok(S3Auth {
    region: auth.credential.region,
    access_key: auth.credential.access_key,
  })
}

pub fn check_headers(parts: &Parts, auth: &AWS4) -> Result<()> {
  for header in parts.headers.keys() {
    if header.as_str().starts_with("x-amz-")
      && !auth
        .signed_headers
        .iter()
        .any(|h| h.eq_ignore_ascii_case(header.as_str()))
    {
      return Err(Error::BadRequest);
    }
  }
  Ok(())
}
