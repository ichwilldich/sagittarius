use axum::{Extension, Router};
use clap::Parser;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use ichwilldich_lib::init::{BaseLayerExt, listener_setup, run_app};

use crate::config::Config;

mod config;
mod test;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let config = Config::parse();
  let listener = listener_setup(&config.base).await;

  let app = Router::new()
    .add_base_layers(&config.base)
    .merge(router(&config).await)
    .layer(Extension(config));

  run_app(listener, app).await;
}

async fn router(config: &Config) -> Router {
  Router::new().nest("/test", test::router(config).await)
}
