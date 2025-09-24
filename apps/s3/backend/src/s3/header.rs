use chrono::NaiveDateTime;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub const DATE_FORMAT: &str = "%Y%m%dT%H%M%SZ";

macro_rules! typed_header {
  ($name:ident, $const:ident, $name_str:literal, $inner:ident) => {
    ichwilldich_lib::typed_header!($name, $const, $name_str, $inner, |s| s.parse().ok(), |v| v
      .to_string());
  };
  ($name:ident, $const:ident, $name_str:literal) => {
    ichwilldich_lib::typed_header!(
      $name,
      $const,
      $name_str,
      String,
      |s| Some(s.to_string()),
      |v| v
    );
  };
}

ichwilldich_lib::typed_header!(
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
  AWZ_CONTENT_LENGTH,
  "x-awz-content-length",
  u64
);

typed_header!(AwzAcl, AWZ_ACL, "x-amz-acl", AwzAclEnum);
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
typed_header!(
  AwzObjectOwnership,
  AWZ_OBJECT_OWNERSHIP,
  "x-amz-object-ownership",
  AwzObjectOwnershipEnum
);

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum AwzContentSha256 {
  UnsignedPayload,
  StreamingUnsignedPayloadTrailer,
  StreamingAws4HmacSha256Payload,
  StreamingAAws4HmacSha256PayloadTrailer,
  #[serde(other)]
  SingleChunk(String),
}

impl AwzContentSha256 {
  pub fn is_chunked(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::StreamingUnsignedPayloadTrailer
        | AwzContentSha256::StreamingAws4HmacSha256Payload
        | AwzContentSha256::StreamingAAws4HmacSha256PayloadTrailer
    )
  }

  pub fn is_trailer(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::StreamingUnsignedPayloadTrailer
        | AwzContentSha256::StreamingAAws4HmacSha256PayloadTrailer
    )
  }

  pub fn is_unsigned(&self) -> bool {
    matches!(
      self,
      AwzContentSha256::UnsignedPayload | AwzContentSha256::StreamingUnsignedPayloadTrailer
    )
  }
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "kebab-case")]
pub enum AwzAclEnum {
  Private,
  PublicRead,
  PublicReadWrite,
  AuthenticatedRead,
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "kebab-case")]
pub enum AwzObjectOwnershipEnum {
  BucketOwnerEnforced,
  BucketOwnerPreferred,
  ObjectWriter,
}
