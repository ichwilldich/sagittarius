use crate::{config::EnvConfig, router_extension};
use axum::Router;

mod auth;
mod header;
pub mod interface;
mod operations;
pub mod storage;

const BUCKET_DIR: &str = "buckets";

pub fn router() -> Router {
  operations::router()
}

router_extension!(
  async fn s3(self, config: &EnvConfig) -> Self {
    use interface::interface;

    self.interface(config).await
  }
);
