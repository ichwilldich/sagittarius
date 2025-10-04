use axum::{Router, extract::Path, routing::put};
use centaurus::{bail, error::Result, path, req::xml::Xml};
use http::HeaderMap;
use serde::Deserialize;

use crate::s3::{
  BUCKET_DIR,
  auth::{Identity, S3Auth},
  interface::S3Interface,
};

pub fn router() -> Router {
  Router::new().route("/{*bucket}", put(create_bucket))
}

/// TODO: Handling of additional header options
async fn create_bucket(
  interface: S3Interface,
  Path(bucket): Path<String>,
  S3Auth { body, identity, .. }: S3Auth<Option<Xml<CreateBucketConfiguration>>>,
) -> Result<HeaderMap> {
  dbg!(&body);
  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} creating bucket {bucket}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to create bucket");
    }
  }

  interface.create_bucket(&bucket).await?;

  let mut headers = HeaderMap::new();
  headers.insert("Location", format!("/{bucket}").parse()?);

  Ok(headers)
}

/// TODO: Handling of additional configuration options
#[derive(Deserialize, Debug)]
struct CreateBucketConfiguration {}
