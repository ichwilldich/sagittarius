use axum::{Router, extract::Path, routing::put};
use http::HeaderMap;
use ichwilldich_lib::{bail, error::Result, path, req::xml::Xml};
use serde::Deserialize;

use crate::s3::{
  auth::{Identity, S3Auth},
  operations::BUCKET_DIR,
  storage::StorageState,
};

pub fn router() -> Router {
  Router::new().route("/{*bucket}", put(create_bucket))
}

/// TODO: Handling of additional header options
async fn create_bucket(
  storage: StorageState,
  Path(bucket): Path<String>,
  S3Auth {
    body: Xml(xml),
    identity,
    ..
  }: S3Auth<Xml<CreateBucketConfiguration>>,
) -> Result<HeaderMap> {
  dbg!(&xml);
  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} creating bucket {bucket}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to create bucket");
    }
  }

  if storage
    .list_dir(&path!(BUCKET_DIR))
    .await?
    .contains(&bucket)
  {
    bail!(CONFLICT, "Bucket {bucket} already exists");
  }

  storage.create_dir(&path!(BUCKET_DIR, &bucket)).await?;

  let mut headers = HeaderMap::new();
  headers.insert("Location", format!("/{bucket}").parse()?);

  Ok(headers)
}

/// TODO: Handling of additional configuration options
#[derive(Deserialize, Debug)]
struct CreateBucketConfiguration {}
