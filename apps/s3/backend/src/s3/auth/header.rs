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
use eyre::{Context, OptionExt};
use futures::StreamExt;
use http::request::Parts;
use ichwilldich_lib::{bail, error::Result};
use memchr::memchr;
use tracing::instrument;

use crate::s3::{
  auth::{
    Identity, S3Auth, SECRET,
    credential::AWS4,
    sig_v4::{CanonicalRequest, Payload, StringToSign},
  },
  header::{AwzContentSha256, AwzContentSha256Header, AwzDate, AwzDecodedContentLength},
};

#[instrument]
pub async fn header_auth(req: Request) -> Result<S3Auth> {
  let (mut parts, body) = req.into_parts();
  let mut auth = parts
    .extract::<TypedHeader<Authorization<AWS4>>>()
    .await
    .ok()
    .map(|TypedHeader(Authorization(auth))| auth);

  let TypedHeader(AwzContentSha256Header(content_hash)) = parts
    .extract::<TypedHeader<AwzContentSha256Header>>()
    .await
    .unwrap_or(TypedHeader(AwzContentSha256Header(
      AwzContentSha256::UnsignedPayload,
    )));

  if !content_hash.is_unsigned() && auth.is_none() {
    bail!(
      FORBIDDEN,
      "Signed payload type requires Authorization header"
    );
  }

  let date = {
    if let Ok(TypedHeader(AwzDate(date))) = parts.extract::<TypedHeader<AwzDate>>().await {
      DateTime::<Utc>::from_naive_utc_and_offset(date, Utc)
    } else if let Ok(TypedHeader(date)) = parts.extract::<TypedHeader<Date>>().await {
      DateTime::<Utc>::from(SystemTime::from(date))
    } else {
      Utc::now()
    }
  };
  if let Some(auth) = &auth {
    check_headers(&parts, auth)?;
  }

  let body = if content_hash.is_chunked() {
    BodyOrBytes::Body(body)
  } else {
    BodyOrBytes::Bytes(Bytes::from_request(Request::from_parts(parts.clone(), body), &()).await?)
  };
  let payload = if let BodyOrBytes::Bytes(bytes) = &body {
    if matches!(content_hash, AwzContentSha256::UnsignedPayload) {
      Payload::Unsigned
    } else if bytes.is_empty() {
      Payload::Empty
    } else {
      Payload::SingleChunk(bytes)
    }
  } else {
    Payload::MultipleChunks
  };

  let signature = if let Some(auth) = &mut auth {
    let signature = CanonicalRequest::new(&parts, auth, &payload)
      .string_to_sign(&date, &auth.credential)
      .sign(SECRET, &auth.credential)?;

    if signature != auth.signature {
      bail!(FORBIDDEN, "Signature mismatch");
    }

    signature
  } else {
    String::new()
  };

  let bytes = match body {
    BodyOrBytes::Body(body) => {
      let body = body.into_data_stream();
      process_chunks(&mut parts, body, &signature, &auth, &date, &content_hash).await?
    }
    BodyOrBytes::Bytes(b) => b,
  };

  Ok(S3Auth {
    identity: Identity::Anonymous,
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
  auth: &Option<AWS4>,
  datetime: &DateTime<Utc>,
  content_hash: &AwzContentSha256,
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
  let trailer = content_hash.is_trailer();

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
        if let Some(auth) = &auth {
          let signature = StringToSign::chunked(datetime, &auth.credential, &last_signature, &d)
            .sign(SECRET, &auth.credential)?;
          if signature != meta.signature {
            bail!(FORBIDDEN, "Chunk signature mismatch");
          }
          last_signature = signature;
        }

        // remove trailing \r\n
        buffer.drain(..2);
        data.extend_from_slice(&d);

        // don't break if there is a trailer chunk and exit automatically when the stream ends
        if meta.length == 0 && trailer {
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

  if trailer {
    let str = str::from_utf8(&buffer).context("Invalid trailer")?;
    let parts = str.split('\n').collect::<Vec<_>>();
    if parts.len() != 2 {
      bail!("Invalid trailer");
    }

    let header_parts = parts[0].split(':').collect::<Vec<_>>();
    if header_parts.len() != 2 {
      bail!("Invalid trailer header");
    }
    let header_name = header_parts[0].trim();
    let header_value = header_parts[1].trim();

    if let Some(auth) = &auth {
      let signature_parts = parts[1].split(':').collect::<Vec<_>>();
      if signature_parts.len() != 2 || signature_parts[0].trim() != "x-amz-trailer-signature" {
        bail!("Invalid trailer signature");
      }
      let expected_signature = signature_parts[1].trim();

      let signature = StringToSign::chunked_trailer(
        datetime,
        &auth.credential,
        &last_signature,
        header_name,
        header_value,
      )
      .sign(SECRET, &auth.credential)?;

      if signature != expected_signature {
        bail!(FORBIDDEN, "Trailer signature mismatch");
      }
    }
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
