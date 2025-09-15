use axum::{Extension, Router, routing::get};
use ichwilldich_lib::FromReqExtension;

use crate::config::Config;

pub async fn router(config: &Config) -> Router {
  Router::new()
    .route("/hello", get(hello))
    .layer(Extension(HelloState {
      message: format!("Hello on port {}", config.base.port),
    }))
}

#[derive(Debug, Clone, FromReqExtension)]
pub struct HelloState {
  pub message: String,
}

async fn hello(state: HelloState) -> String {
  state.message
}
