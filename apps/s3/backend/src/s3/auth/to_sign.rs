use chrono::{DateTime, Utc};
use http::request::Parts;
use sha2::{Digest, Sha256};

use crate::s3::auth::{AWS4, AWS4Credential};

// SHA256 hash of an empty string
const EMPTY_STRING_SHA256_HASH: &str =
  "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub enum Payload<'a> {
  Unsigned,
  Empty,
  SingleChunk(&'a [u8]),
  MultipleChunks,
}

#[derive(Debug)]
pub struct CanonicalRequest(String);

pub fn canonical_request(parts: &Parts, auth: &mut AWS4, payload: &Payload) -> CanonicalRequest {
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
      .collect();
    query_pairs.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    let canonical_query: String = query_pairs
      .into_iter()
      .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
      .collect::<Vec<String>>()
      .join("&");
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
    Payload::Empty => "EMPTY-PAYLOAD",
    Payload::SingleChunk(data) => &hex::encode(Sha256::digest(data)),
    Payload::MultipleChunks => "MULTIPLE-CHUNKS-PAYLOAD",
  };
  req.push_str(str);

  CanonicalRequest(req)
}

pub fn string_to_sign(
  canonical_request: &CanonicalRequest,
  amz_date: &DateTime<Utc>,
  credential: &AWS4Credential,
) -> String {
  let mut string_to_sign = String::new();
  // Algorithm
  string_to_sign.push_str("AWS4-HMAC-SHA256\n");

  // RequestDateTime
  string_to_sign.push_str(&amz_date.format("%Y%m%dT%H%M%SZ").to_string());
  string_to_sign.push('\n');

  // CredentialScope
  string_to_sign.push_str(&format!(
    "{}/{}/s3/aws4_request\n",
    credential.date, credential.region
  ));

  // HashedCanonicalRequest
  string_to_sign.push_str(&hex::encode(Sha256::digest(canonical_request.0.as_bytes())));
  string_to_sign
}
