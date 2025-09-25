use axum::extract::Request;
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::StreamExt;
use ichwilldich_lib::{bail, error::Result};
use tracing::instrument;

use crate::s3::{
  auth::{
    Identity, S3Auth, SECRET,
    body::{Body, BodyWriter},
    credential::AWS4,
    header::check_headers,
    sig_v4::{ALGORITHM, CanonicalRequest, Payload},
  },
  header::DATE_FORMAT,
};

#[instrument]
pub async fn query_auth<T: Body>(req: Request, query: &[(String, String)]) -> Result<S3Auth<T>> {
  let mut data = parse_query(query)?;
  let (parts, body) = req.into_parts();

  check_headers(&parts, &data.auth)?;
  let date = DateTime::<Utc>::from_naive_utc_and_offset(data.date, Utc);
  let signature = CanonicalRequest::new(&parts, &mut data.auth, &Payload::Unsigned)
    .string_to_sign(&date, &data.auth.credential)
    .sign(SECRET, &data.auth.credential)?;

  if signature != data.auth.signature {
    bail!(FORBIDDEN, "Signature mismatch");
  }

  let mut writer = T::Writer::new().await?;
  let mut stream = body.into_data_stream();

  while let Some(chunk) = stream.next().await {
    if let Ok(chunk) = chunk {
      writer.write(&chunk).await?;
    } else {
      bail!(INTERNAL_SERVER_ERROR, "Error reading body");
    }
  }

  Ok(S3Auth {
    identity: Identity::AccessKey(data.auth.credential.access_key),
    body: T::from_writer(writer)?,
    additional: None,
  })
}

struct QueryData {
  algorithm: String,
  date: NaiveDateTime,
  auth: AWS4,
}

#[instrument]
fn parse_query(query: &[(String, String)]) -> Result<QueryData> {
  let mut algorithm = None;
  let mut credential = None;
  let mut date = None;
  let mut expires = None;
  let mut signed_headers = None;
  let mut signature = None;

  for (k, v) in query {
    match k.as_str() {
      "X-Amz-Algorithm" => algorithm = Some(v.to_string()),
      "X-Amz-Credential" => credential = Some(v.parse()?),
      "X-Amz-Date" => date = Some(NaiveDateTime::parse_from_str(v, DATE_FORMAT)?),
      "X-Amz-Expires" => expires = Some(v.parse()?),
      "X-Amz-SignedHeaders" => {
        let headers = v.split(';').map(|s| s.to_string()).collect::<Vec<_>>();
        if !headers.iter().any(|s| s.eq_ignore_ascii_case("host")) {
          bail!("SignedHeaders must contain 'host'");
        }

        signed_headers = Some(headers)
      }
      "X-Amz-Signature" => signature = Some(v.to_string()),
      _ => {}
    }
  }

  if algorithm.is_none()
    || credential.is_none()
    || date.is_none()
    || expires.is_none()
    || signed_headers.is_none()
    || signature.is_none()
  {
    bail!(
      "Missing required query parameters algorithm: {algorithm:?}, credential: {credential:?}, date: {date:?}, expires: {expires:?}, signed_headers: {signed_headers:?}, signature: {signature:?}"
    );
  }

  let expires = expires.unwrap();
  // Expires must be between 1 and 604800 seconds (7 days)
  if expires == 0 || expires > 604800 {
    bail!(
      FORBIDDEN,
      "X-Amz-Expires must be between 1 and 604800 seconds"
    );
  }

  let date = date.unwrap();
  if (Utc::now().naive_utc() - date).num_seconds() > expires {
    bail!(FORBIDDEN, "Request has expired");
  }

  let data = QueryData {
    algorithm: algorithm.unwrap(),
    date,
    auth: AWS4 {
      credential: credential.unwrap(),
      signed_headers: signed_headers.unwrap(),
      signature: signature.unwrap(),
    },
  };

  if data.algorithm != ALGORITHM {
    bail!("Only AWS4-HMAC-SHA256 is supported");
  }

  Ok(data)
}
