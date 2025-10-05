use axum::{
  Json, Router,
  extract::Path,
  routing::{delete, get, put},
};
use centaurus::{error::Result, req::xml::Xml};
use http::HeaderMap;
use serde::Deserialize;

use crate::s3::{
  auth::{Identity, S3Auth},
  interface::S3Interface,
};

pub fn router() -> Router {
  Router::new()
    .route("/{*bucket}", put(create_bucket))
    .route("/{*bucket}", delete(delete_bucket))
    .route("/", get(list_buckets))
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

async fn delete_bucket(
  interface: S3Interface,
  Path(bucket): Path<String>,
  S3Auth { identity, .. }: S3Auth<()>,
) -> Result<()> {
  let bucket = bucket.trim_end_matches('/').to_string();

  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} deleting bucket {bucket}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to delete bucket");
    }
  }

  interface.delete_bucket(&bucket).await?;

  Ok(())
}

async fn list_buckets(interface: S3Interface) -> Result<Json<Vec<String>>> {
  let buckets = interface.list_buckets().await?;
  Ok(Json(buckets))
}

/// TODO: Handling of additional configuration options
#[derive(Deserialize, Debug)]
struct CreateBucketConfiguration {}
