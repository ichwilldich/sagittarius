use std::net::SocketAddr;

use axum::{Router, serve};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::BaseConfig, init::cors::cors, router_extension};

mod cors;
mod logging;

pub fn init_logging(config: &BaseConfig) {
  color_eyre::install().expect("Failed to install color_eyre");

  let filter_layer: LevelFilter = config.log_level.into();

  let layer = tracing_subscriber::fmt::layer()
    .with_ansi(true)
    .with_filter(filter_layer);

  tracing_subscriber::registry()
    .with(layer)
    .with(ErrorLayer::default())
    .init();
}

pub async fn listener_setup(port: u16) -> TcpListener {
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  TcpListener::bind(addr)
    .await
    .expect("Failed to bind to address")
}

pub async fn run_app(listener: TcpListener, app: Router) {
  serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server");
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}

router_extension!(
  async fn add_base_layers(self, config: &BaseConfig) -> Self {
    use logging::logging;

    self
      .layer(ServiceBuilder::new().layer(cors(config).expect("Failed to build CORS layer")))
      .logging()
      .await
  }
);
