use axum::{Router, extract::Path, routing::put};
use centaurus::{error::Result, req::xml::Xml};
use http::HeaderMap;
use serde::Deserialize;

use crate::s3::{
  auth::{Identity, S3Auth},
  interface::S3Interface,
};

pub fn router() -> Router {
  Router::new().route("/{bucket}/{*object}", put(put_object))
}

async fn put_object(
  interface: S3Interface,
  Path((bucket, object)): Path<(String, String)>,
  S3Auth { identity, .. }: S3Auth<Option<Xml<PutObjectConfiguration>>>,
) -> Result<HeaderMap> {
  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} putting object {object}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to put object");
    }
  }

  tracing::info!(
    "put_object handler triggered: bucket={} object={}",
    bucket,
    object
  );

  interface.put_object(&bucket, &object).await?;

  let headers = HeaderMap::new();

  Ok(headers)
}

#[derive(Deserialize, Debug)]
struct PutObjectConfiguration {}
