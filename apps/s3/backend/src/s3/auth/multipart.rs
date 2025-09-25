use axum::extract::{FromRequest, Multipart, Request};
use base64::prelude::*;
use chrono::NaiveDateTime;
use eyre::{Context, OptionExt};
use ichwilldich_lib::{bail, error::Result};
use tracing::instrument;

use crate::s3::{
  auth::{
    Identity, S3Auth, SECRET,
    body::Body,
    credential::AWS4Credential,
    sig_v4::{ALGORITHM, StringToSign},
  },
  header::DATE_FORMAT,
};

#[instrument]
pub async fn multipart_auth<T: Body>(req: Request) -> Result<S3Auth<T>> {
  let multipart = Multipart::from_request(req, &()).await?;
  let data = parse_multipart(multipart).await?;

  let signature = StringToSign::new(data.policy).sign(SECRET, &data.credential)?;

  if signature != data.signature {
    bail!(FORBIDDEN, "Signature mismatch");
  }

  Ok(S3Auth {
    identity: Identity::AccessKey(data.credential.access_key),
    body: todo!(),
  })
}

struct MultipartData {
  policy: String,
  algorithm: String,
  credential: AWS4Credential,
  _date: NaiveDateTime,
  signature: String,
}

#[instrument]
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
    policy: policy.ok_or_eyre("Missing policy field")?,
    algorithm: algorithm.ok_or_eyre("Missing algorithm field")?,
    credential: credential.ok_or_eyre("Missing credential field")?.parse()?,
    _date: NaiveDateTime::parse_from_str(&date.ok_or_eyre("Missing date field")?, DATE_FORMAT)?,
    signature: signature.ok_or_eyre("Missing signature field")?,
  };

  // check if policy is valid base64
  BASE64_STANDARD
    .decode(&multipart_data.policy)
    .context("Invalid policy field")?;

  // check algorithm
  if multipart_data.algorithm != ALGORITHM {
    bail!("Only AWS4-HMAC-SHA256 is supported");
  }

  Ok(multipart_data)
}
