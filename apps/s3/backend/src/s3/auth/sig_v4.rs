use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use http::request::Parts;
use ichwilldich_lib::error::Result;
use sha2::{Digest, Sha256};
use tracing::instrument;

use crate::s3::{
  auth::credential::{AWS4, AWS4Credential},
  header::DATE_FORMAT,
};

// SHA256 hash of an empty string
const EMPTY_STRING_SHA256_HASH: &str =
  "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub const ALGORITHM: &str = "AWS4-HMAC-SHA256";
pub const ALGORITHM_CHUNKED: &str = "AWS4-HMAC-SHA256-PAYLOAD";

#[derive(Debug)]
pub enum Payload<'a> {
  Unsigned,
  Empty,
  SingleChunk(&'a [u8]),
  MultipleChunks,
}

#[derive(Debug)]
pub struct CanonicalRequest(String);

impl CanonicalRequest {
  #[instrument]
  pub fn new(parts: &Parts, auth: &mut AWS4, payload: &Payload) -> CanonicalRequest {
    let mut req = String::new();
    // HTTPMethod
    req.push_str(parts.method.as_str());
    req.push('\n');

    // CanonicalURI
    req.push_str(parts.uri.path());
    req.push('\n');

    // CanonicalQueryString
    if let Some(query) = parts.uri.query() {
      let mut query_pairs: Vec<_> = query
        .split('&')
        .map(|pair| {
          let mut split = pair.splitn(2, '=');
          let key = split.next().unwrap_or("");
          let value = split.next().unwrap_or("");
          (key, value)
        })
        .filter(|(k, _)| k != &"X-Amz-Signature")
        .collect();
      query_pairs.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
      let canonical_query = serde_urlencoded::to_string(&query_pairs).unwrap_or_default();
      req.push_str(&canonical_query);
    }
    req.push('\n');

    // CanonicalHeaders
    let mut headers: Vec<_> = parts
      .headers
      .iter()
      .map(|(k, v)| {
        (
          k.as_str().to_ascii_lowercase(),
          v.to_str().unwrap_or("").trim(),
        )
      })
      .filter(|(k, _)| {
        auth
          .signed_headers
          .iter()
          .any(|h| h.eq_ignore_ascii_case(k))
      })
      .collect();
    headers.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    for (header, value) in &headers {
      req.push_str(&format!("{header}:{value}\n"));
    }
    req.push('\n');

    // SignedHeaders
    auth.signed_headers.sort_unstable();
    let signed_headers: String = auth.signed_headers.join(";");
    req.push_str(&format!("{}\n", signed_headers));

    // Payload
    let str = match payload {
      Payload::Unsigned => "UNSIGNED-PAYLOAD",
      Payload::Empty => EMPTY_STRING_SHA256_HASH,
      Payload::SingleChunk(data) => &hex::encode(Sha256::digest(data)),
      Payload::MultipleChunks => "STREAMING-AWS4-HMAC-SHA256-PAYLOAD",
    };
    req.push_str(str);

    CanonicalRequest(req)
  }

  #[instrument]
  pub fn string_to_sign(
    &self,
    amz_date: &DateTime<Utc>,
    credential: &AWS4Credential,
  ) -> StringToSign {
    let mut string_to_sign = String::new();
    // Algorithm
    string_to_sign.push_str(ALGORITHM);
    string_to_sign.push('\n');

    // RequestDateTime
    string_to_sign.push_str(&amz_date.format(DATE_FORMAT).to_string());
    string_to_sign.push('\n');

    // CredentialScope
    string_to_sign.push_str(&format!(
      "{}/{}/s3/aws4_request\n",
      credential.date, credential.region
    ));

    // HashedCanonicalRequest
    string_to_sign.push_str(&hex::encode(Sha256::digest(self.0.as_bytes())));

    StringToSign(string_to_sign)
  }
}

#[derive(Debug)]
pub struct StringToSign(String);

impl StringToSign {
  pub fn new(s: String) -> Self {
    Self(s)
  }

  pub fn chunked(
    datetime: &DateTime<Utc>,
    credential: &AWS4Credential,
    previous_signature: &str,
    current_data: &[u8],
  ) -> Self {
    let mut string_to_sign = Self::chunked_base(datetime, credential, previous_signature);

    // HashedEmptyString
    string_to_sign.push_str(EMPTY_STRING_SHA256_HASH);
    string_to_sign.push('\n');

    // HashedChunkData
    string_to_sign.push_str(&hex::encode(Sha256::digest(current_data)));

    StringToSign(string_to_sign)
  }

  pub fn chunked_trailer(
    datetime: &DateTime<Utc>,
    credential: &AWS4Credential,
    previous_signature: &str,
    tailing_header_name: &str,
    trailing_header_value: &str,
  ) -> Self {
    let mut string_to_sign = Self::chunked_base(datetime, credential, previous_signature);

    // Trailing checksum header
    string_to_sign.push_str(&format!("{tailing_header_name}:{trailing_header_value}\n"));

    StringToSign(string_to_sign)
  }

  fn chunked_base(
    datetime: &DateTime<Utc>,
    credential: &AWS4Credential,
    previous_signature: &str,
  ) -> String {
    let mut string_to_sign = String::new();
    // Algorithm
    string_to_sign.push_str(ALGORITHM_CHUNKED);
    string_to_sign.push('\n');

    // RequestDateTime
    string_to_sign.push_str(&datetime.format(DATE_FORMAT).to_string());
    string_to_sign.push('\n');

    // CredentialScope
    string_to_sign.push_str(&format!(
      "{}/{}/s3/aws4_request\n",
      credential.date, credential.region
    ));

    // PreviousSignature
    string_to_sign.push_str(previous_signature);
    string_to_sign.push('\n');

    string_to_sign
  }

  #[instrument]
  pub fn sign(&self, secret_key: &str, credential: &AWS4Credential) -> Result<String> {
    let date_key = hmac(format!("AWS4{}", secret_key).as_bytes(), &credential.date)?;
    let date_region_key = hmac(&date_key, &credential.region)?;
    let date_region_service_key = hmac(&date_region_key, "s3")?;
    let signing_key = hmac(&date_region_service_key, "aws4_request")?;
    let signature = hmac(&signing_key, &self.0)?;
    Ok(hex::encode(signature))
  }
}

fn hmac(key: &[u8], msg: &str) -> Result<Vec<u8>> {
  let mut hmac = Hmac::<Sha256>::new_from_slice(key)?;
  hmac.update(msg.as_bytes());
  Ok(hmac.finalize().into_bytes().to_vec())
}
