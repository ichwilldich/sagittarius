use axum::http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

use crate::{config::Config, error::Result};

pub fn cors(config: &Config) -> Result<CorsLayer> {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_credentials(true);

  let mut origins = Vec::new();
  for origin in config.allowed_origins.split(',') {
    origins.push(origin.parse::<HeaderValue>()?);
  }

  Ok(cors.allow_origin(origins))
}
