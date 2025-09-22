use axum::extract::Request;
use chrono::{DateTime, NaiveDateTime, Utc};
use ichwilldich_lib::error::{Error, Result};

use crate::s3::{
  auth::{
    S3Auth, SECRET,
    credential::AWS4,
    header::check_headers,
    sig_v4::{ALGORITHM, CanonicalRequest, Payload},
  },
  header::DATE_FORMAT,
};

pub async fn query_auth(req: Request, query: &[(String, String)]) -> Result<S3Auth> {
  let mut data = parse_query(query)?;
  let (parts, _body) = req.into_parts();

  check_headers(&parts, &data.auth)?;
  let date = DateTime::<Utc>::from_naive_utc_and_offset(data.date, Utc);
  let signature = CanonicalRequest::new(&parts, &mut data.auth, &Payload::Unsigned)
    .string_to_sign(&date, &data.auth.credential)
    .sign(SECRET, &data.auth.credential)?;

  if signature != data.auth.signature {
    return Err(Error::Forbidden);
  }

  Ok(S3Auth {
    region: data.auth.credential.region,
    access_key: data.auth.credential.access_key,
  })
}

struct QueryData {
  algorithm: String,
  date: NaiveDateTime,
  expires: u32,
  auth: AWS4,
}

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
          return Err(Error::BadRequest);
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
    return Err(Error::BadRequest);
  }

  let data = QueryData {
    algorithm: algorithm.unwrap(),
    date: date.unwrap(),
    expires: expires.unwrap(),
    auth: AWS4 {
      credential: credential.unwrap(),
      signed_headers: signed_headers.unwrap(),
      signature: signature.unwrap(),
    },
  };

  if data.algorithm != ALGORITHM {
    return Err(Error::BadRequest);
  }

  Ok(data)
}
