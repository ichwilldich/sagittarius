use axum::{Extension, Router, routing::get};
use ichwilldich_lib::{FromReqExtension, router_extension};

use crate::config::Config;

pub fn router() -> Router {
  Router::new().route("/hello", get(hello))
}

router_extension!(
  async fn test(self, config: &Config) -> Self {
    self.layer(Extension(HelloState {
      message: format!("Hello on port {}", config.base.port),
    }))
  }
);

#[derive(Debug, Clone, FromReqExtension)]
struct HelloState {
  message: String,
}

async fn hello(state: HelloState) -> String {
  state.message
}
