use axum::extract::{FromRequest, Multipart, Request};
use base64::prelude::*;
use chrono::NaiveDateTime;
use ichwilldich_lib::error::{Error, Result};

use crate::s3::{
  auth::{
    S3Auth, SECRET,
    credential::AWS4Credential,
    sig_v4::{ALGORITHM, StringToSign},
  },
  header::DATE_FORMAT,
};

pub async fn multipart_auth(req: Request) -> Result<S3Auth> {
  let multipart = Multipart::from_request(req, &()).await?;
  let data = parse_multipart(multipart).await?;

  let signature = StringToSign::new(data.policy).sign(SECRET, &data.credential)?;

  if signature != data.signature {
    return Err(Error::Forbidden);
  }

  Ok(S3Auth {
    region: data.credential.region,
    access_key: data.credential.access_key,
  })
}

struct MultipartData {
  policy: String,
  algorithm: String,
  credential: AWS4Credential,
  _date: NaiveDateTime,
  signature: String,
}

async fn parse_multipart(mut multipart: Multipart) -> Result<MultipartData> {
  let mut policy = None;
  let mut algorithm = None;
  let mut credential = None;
  let mut date = None;
  let mut signature = None;

  while let Some(field) = multipart.next_field().await? {
    let name = field.name().unwrap_or("").to_string();
    let value = field.text().await?;
    match name.as_str() {
      "policy" => policy = Some(value),
      "x-amz-algorithm" => algorithm = Some(value),
      "x-amz-credential" => credential = Some(value),
      "x-amz-date" => date = Some(value),
      "x-amz-signature" => signature = Some(value),
      _ => {}
    }
  }

  let multipart_data = MultipartData {
    policy: policy.ok_or(Error::BadRequest)?,
    algorithm: algorithm.ok_or(Error::BadRequest)?,
    credential: credential.ok_or(Error::BadRequest)?.parse()?,
    _date: NaiveDateTime::parse_from_str(&date.ok_or(Error::BadRequest)?, DATE_FORMAT)?,
    signature: signature.ok_or(Error::BadRequest)?,
  };

  // check if policy is valid base64
  BASE64_STANDARD
    .decode(&multipart_data.policy)
    .map_err(|_| Error::BadRequest)?;

  // check algorithm
  if multipart_data.algorithm != ALGORITHM {
    return Err(Error::NotImplemented);
  }

  Ok(multipart_data)
}
