use chrono::NaiveDateTime;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub const DATE_FORMAT: &str = "%Y%m%dT%H%M%SZ";

macro_rules! typed_header {
  ($name:ident, $const:ident, $name_str:literal, $inner:ident) => {
    centaurus::typed_header!($name, $const, $name_str, $inner, |s| s.parse().ok(), |v| v
      .to_string());
  };
  ($name:ident, $const:ident, $name_str:literal) => {
    centaurus::typed_header!(
      $name,
      $const,
      $name_str,
      String,
      |s| Some(s.to_string()),
      |v| v
    );
  };
}

centaurus::typed_header!(
  AwzDate,
  AWZ_DATE,
  "x-amz-date",
  NaiveDateTime,
  |s| NaiveDateTime::parse_from_str(s, DATE_FORMAT).ok(),
  |v| v.format(DATE_FORMAT).to_string()
);

typed_header!(
  AwzContentSha256Header,
  AWZ_CONTENT_SHA256,
  "x-amz-content-sha256",
  AwzContentSha256
);

typed_header!(
  AwzDecodedContentLength,
  AWZ_DECODED_CONTENT_LENGTH,
  "x-awz-decoded-content-length",
  u64
);

typed_header!(
  AwzBucketObjectLockEnabled,
  AWZ_BUCKET_OBJECT_LOCK_ENABLED,
  "x-amz-bucket-object-lock-enabled",
  bool
);
typed_header!(
  AwzGrantFullControl,
  AWZ_GRANT_FULL_CONTROL,
  "x-amz-grant-full-control"
);
typed_header!(AwzGrantRead, AWZ_GRANT_READ, "x-amz-grant-read");
typed_header!(AwzGrantReadAcp, AWZ_GRANT_READ_ACP, "x-amz-grant-read-acp");
typed_header!(AwzGrantWrite, AWZ_GRANT_WRITE, "x-amz-grant-write");
typed_header!(
  AwzGrantWriteAcp,
  AWZ_GRANT_WRITE_ACP,
  "x-amz-grant-write-acp"
);

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum AwzContentSha256 {
  UnsignedPayload,
  StreamingUnsignedPayloadTrailer,
  StreamingAws4HmacSha256Payload,
  StreamingAws4HmacSha256PayloadTrailer,
  #[serde(other)]
  SingleChunk(String),
}

impl AwzContentSha256 {
  pub fn is_chunked(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::StreamingUnsignedPayloadTrailer
        | AwzContentSha256::StreamingAws4HmacSha256Payload
        | AwzContentSha256::StreamingAws4HmacSha256PayloadTrailer
    )
  }

  pub fn is_trailer(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::StreamingUnsignedPayloadTrailer
        | AwzContentSha256::StreamingAws4HmacSha256PayloadTrailer
    )
  }

  pub fn is_unsigned(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::UnsignedPayload | AwzContentSha256::StreamingUnsignedPayloadTrailer
    )
  }
}

#[cfg(test)]
mod test {
  use axum_extra::headers::Header;
  use chrono::{NaiveDate, NaiveTime};
  use http::HeaderValue;

  use super::*;

  #[test]
  fn test_awz_date() {
    let value = HeaderValue::from_static("20230915T123456Z");
    let date = AwzDate::decode(&mut std::iter::once(&value)).unwrap();
    assert_eq!(
      date.0,
      NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 9, 15).unwrap(),
        NaiveTime::from_hms_opt(12, 34, 56).unwrap()
      )
    );
  }

  #[test]
  fn test_awz_content_sha256() {
    for (raw, expected) in [
      ("UNSIGNED-PAYLOAD", AwzContentSha256::UnsignedPayload),
      (
        "STREAMING-UNSIGNED-PAYLOAD-TRAILER",
        AwzContentSha256::StreamingUnsignedPayloadTrailer,
      ),
      (
        "STREAMING-AWS4-HMAC-SHA256-PAYLOAD",
        AwzContentSha256::StreamingAws4HmacSha256Payload,
      ),
      (
        "STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER",
        AwzContentSha256::StreamingAws4HmacSha256PayloadTrailer,
      ),
      (
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        AwzContentSha256::SingleChunk(
          "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        ),
      ),
    ] {
      let value = HeaderValue::from_static(raw);
      let sha256 = AwzContentSha256Header::decode(&mut std::iter::once(&value)).unwrap();
      assert_eq!(sha256.0, expected);

      if raw.contains("STREAMING") {
        assert!(sha256.0.is_chunked());
      } else {
        assert!(!sha256.0.is_chunked());
      }

      if raw.contains("TRAILER") {
        assert!(sha256.0.is_trailer());
      } else {
        assert!(!sha256.0.is_trailer());
      }

      if raw.contains("UNSIGNED") {
        assert!(sha256.0.is_unsigned());
      } else {
        assert!(!sha256.0.is_unsigned());
      }
    }
  }
}
