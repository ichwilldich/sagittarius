use axum::Router;
use ichwilldich_lib::router_extension;

use crate::config::Config;

mod header;
mod operations;
pub mod storage;

pub fn router() -> Router {
  operations::router()
}

router_extension!(
  async fn s3(self, config: &Config) -> Self {
    use storage::storage;

    self.storage(config).await
  }
);
