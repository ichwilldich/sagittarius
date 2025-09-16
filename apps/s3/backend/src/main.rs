use axum::{Extension, Router};
use clap::Parser;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use ichwilldich_lib::{
  init::{add_base_layers, init_logging, listener_setup, run_app},
  router_extension,
};

use crate::config::Config;

mod config;
mod s3;
mod test;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let config = Config::parse();
  init_logging(&config.base);

  let listener = listener_setup(&config.base).await;

  let app = router()
    .add_base_layers(&config.base)
    .await
    .state(&config)
    .await
    .layer(Extension(config));

  run_app(listener, app).await;
}

fn router() -> Router {
  Router::new().nest("/test", test::router())
}

router_extension!(
  async fn state(self, config: &Config) -> Self {
    use test::test;

    self.test(config).await
  }
);
