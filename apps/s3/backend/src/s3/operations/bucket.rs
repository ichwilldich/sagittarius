use axum::{Router, extract::Path, routing::put};
use http::HeaderMap;
use ichwilldich_lib::{error::{Error, Result}, path, req::xml::Xml};
use serde::Deserialize;

use crate::s3::{operations::BUCKET_DIR, storage::StorageState};

pub fn router() -> Router {
  Router::new().route("/{bucket}", put(create_bucket))
}

/// TODO: Handling of additional header options
async fn create_bucket(
  storage: StorageState,
  Path(bucket): Path<String>,
  Xml(_req): Xml<CreateBucketConfiguration>,
) -> Result<HeaderMap> {
  if storage.list_dir(&path!(BUCKET_DIR)).await?.contains(&bucket) {
    return Err(Error::Conflict);
  }

  storage.create_dir(&path!(BUCKET_DIR, &bucket)).await?;

  let mut headers = HeaderMap::new();
  headers.insert("Location", format!("/{bucket}").parse()?);

  Ok(headers)
}

/// TODO: Handling of additional configuration options
#[derive(Deserialize)]
struct CreateBucketConfiguration {}
