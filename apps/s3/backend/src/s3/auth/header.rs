use std::time::SystemTime;

use axum::{
  RequestPartsExt,
  body::{Body, BodyDataStream, Bytes},
  extract::{FromRequest, Request},
};
use axum_extra::{
  TypedHeader,
  headers::{Authorization, ContentEncoding, Date},
};
use chrono::{DateTime, Utc};
use eyre::OptionExt;
use futures::StreamExt;
use http::request::Parts;
use ichwilldich_lib::{bail, error::Result};
use memchr::memchr;
use tracing::instrument;

use crate::s3::{
  auth::{
    S3Auth, SECRET,
    credential::{self, AWS4, AWS4Credential},
    sig_v4::{CanonicalRequest, Payload, StringToSign},
  },
  header::{AwzContentSha256, AwzContentSha256Header, AwzDate, AwzDecodedContentLength},
};

#[instrument]
pub async fn header_auth(req: Request) -> Result<S3Auth> {
  let (mut parts, body) = req.into_parts();
  let TypedHeader(Authorization(mut auth)) =
    parts.extract::<TypedHeader<Authorization<AWS4>>>().await?;
  let TypedHeader(AwzContentSha256Header(content_hash)) = parts
    .extract::<TypedHeader<AwzContentSha256Header>>()
    .await?;

  let date = {
    if let Ok(TypedHeader(AwzDate(date))) = parts.extract::<TypedHeader<AwzDate>>().await {
      DateTime::<Utc>::from_naive_utc_and_offset(date, Utc)
    } else if let Ok(TypedHeader(date)) = parts.extract::<TypedHeader<Date>>().await {
      DateTime::<Utc>::from(SystemTime::from(date))
    } else {
      Utc::now()
    }
  };
  check_headers(&parts, &auth)?;

  let body = if content_hash.is_chunked() {
    BodyOrBytes::Body(body)
  } else {
    BodyOrBytes::Bytes(Bytes::from_request(Request::from_parts(parts.clone(), body), &()).await?)
  };
  let payload = if let BodyOrBytes::Bytes(bytes) = &body {
    if bytes.is_empty() {
      Payload::Empty
    } else if matches!(content_hash, AwzContentSha256::UnsignedPayload) {
      Payload::Unsigned
    } else {
      Payload::SingleChunk(bytes)
    }
  } else {
    Payload::MultipleChunks
  };

  let signature = CanonicalRequest::new(&parts, &mut auth, &payload)
    .string_to_sign(&date, &auth.credential)
    .sign(SECRET, &auth.credential)?;

  if signature != auth.signature {
    bail!(FORBIDDEN, "Signature mismatch");
  }

  let bytes = match body {
    BodyOrBytes::Body(body) => {
      let body = body.into_data_stream();
      process_chunks(&mut parts, body, &signature, &auth.credential, &date).await?
    }
    BodyOrBytes::Bytes(b) => b,
  };

  Ok(S3Auth {
    region: auth.credential.region,
    access_key: auth.credential.access_key,
  })
}

enum BodyOrBytes {
  Body(Body),
  Bytes(Bytes),
}

#[instrument]
async fn process_chunks(
  parts: &mut Parts,
  mut body: BodyDataStream,
  initial_signature: &str,
  credential: &AWS4Credential,
  datetime: &DateTime<Utc>,
) -> Result<Bytes> {
  let TypedHeader(encoding) = parts.extract::<TypedHeader<ContentEncoding>>().await?;
  if !encoding.contains("aws-chunked") {
    bail!("Content-Encoding must be 'aws-chunked'");
  }

  let TypedHeader(AwzDecodedContentLength(length)) = parts
    .extract::<TypedHeader<AwzDecodedContentLength>>()
    .await?;

  let mut buffer = Vec::new();
  let mut current_meta: Option<ChunkMeta> = None;
  let mut data = Vec::new();
  let mut last_signature = initial_signature.to_string();

  while let Some(chunk) = body.next().await {
    let Ok(chunk) = chunk else {
      bail!("Invalid chunk");
    };

    buffer.extend_from_slice(&chunk);

    if let Some(meta) = &current_meta {
      // +2 for trailing \r\n
      if buffer.len() >= meta.length + 2 {
        let d = buffer.drain(..meta.length).collect::<Vec<u8>>();
        if buffer.len() < 2 || &buffer[..2] != b"\r\n" {
          bail!("Invalid chunk ending");
        }

        // verify chunk signature
        let signature = StringToSign::chunked(datetime, credential, &last_signature, &d)
          .sign(SECRET, credential)?;
        if signature != meta.signature {
          bail!(FORBIDDEN, "Chunk signature mismatch");
        }
        last_signature = signature;

        // remove trailing \r\n
        buffer.drain(..2);
        data.extend_from_slice(&d);

        if meta.length == 0 {
          break;
        }
        current_meta = None;
      }
    }

    if let Some(i) = memchr(b'\n', &buffer) {
      let line = String::from_utf8_lossy(&buffer[..i]);
      let mut parts = line.split(';');
      let length = parts
        .next()
        .and_then(|s| usize::from_str_radix(s, 16).ok())
        .ok_or_eyre("Invalid chunk length")?;

      let signature = parts
        .find_map(|s| s.strip_prefix("chunk-signature="))
        .map(|s| s.to_string())
        .ok_or_eyre("Missing chunk signature")?;

      current_meta = Some(ChunkMeta { length, signature });
      buffer.drain(..=i);
    } else {
      continue;
    }
  }

  if data.len() != length as usize {
    bail!("Decoded content length mismatch");
  }

  Ok(Bytes::from(data))
}

struct ChunkMeta {
  length: usize,
  signature: String,
}

#[instrument]
pub fn check_headers(parts: &Parts, auth: &AWS4) -> Result<()> {
  for header in parts.headers.keys() {
    if header.as_str().starts_with("x-amz-")
      && !auth
        .signed_headers
        .iter()
        .any(|h| h.eq_ignore_ascii_case(header.as_str()))
    {
      bail!("SignedHeaders missing header {}", header.as_str());
    }
  }
  Ok(())
}
