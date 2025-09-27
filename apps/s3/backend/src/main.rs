use axum::{Extension, Router};
use clap::Parser;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use ichwilldich_lib::init::{add_base_layers, init_logging, listener_setup, run_app};
use tokio::join;
use tracing::info;

use crate::{config::Config, macros::DualRouterExt};

mod auth;
mod config;
mod db;
mod example;
mod macros;
mod s3;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let config = Config::parse();
  init_logging(&config.base);

  let app_listener = listener_setup(config.base.port).await;
  let s3_listener = listener_setup(config.s3_port).await;

  let (app, s3) = (router(&config).await, s3_router(&config).await)
    .state(&config)
    .await
    .layer(Extension(config));

  info!("Starting s3 sever");
  join!(run_app(app_listener, app), run_app(s3_listener, s3));
}

async fn router(config: &Config) -> Router {
  auth::router()
    .merge(example::router())
    .add_base_layers(&config.base)
    .await
}

async fn s3_router(config: &Config) -> Router {
  s3::router().add_base_layers(&config.base).await
}

router_extension!(
  async fn state(self, config: &Config) -> Self {
    use auth::auth;
    use s3::s3;

    let db = db::init_db(config).await;

    self
      .s3(config)
      .await
      .auth(config, &db)
      .await
      .layer(Extension(db))
  }
);

#[cfg(test)]
mod test {
  use clap::Parser;

  #[tokio::test]
  async fn test_router() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
    }
    // test if there are any handler setup error that are not caught at compile time
    let _ = super::router(&super::Config::parse_from([""])).await;
  }
}
