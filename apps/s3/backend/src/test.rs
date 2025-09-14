use axum::{Extension, Router, routing::get};

use crate::{config::Config, from_req_extension, state_trait};

pub fn router() -> Router {
  Router::new().route("/hello", get(hello))
}

state_trait!(
  async fn test(self, config: &Config) -> Self {
    self.layer(Extension(HelloState {
      message: format!("Hello on port {}", config.port),
    }))
  }
);

#[derive(Debug, Clone)]
struct HelloState {
  message: String,
}
from_req_extension!(HelloState);

async fn hello(state: HelloState) -> String {
  state.message
}
