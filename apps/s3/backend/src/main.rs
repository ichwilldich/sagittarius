use std::net::SocketAddr;

use axum::{Router, serve};
use clap::Parser;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;

use crate::{config::Config, cors::cors};

mod config;
mod cors;
mod error;
mod logging;
mod macros;
mod test;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let config = Config::parse();

  tracing_subscriber::fmt()
    .with_max_level(config.log_level)
    .init();

  let app = Router::new()
    .merge(routes())
    .state(&config)
    .await
    .layer(ServiceBuilder::new().layer(cors(&config).expect("Failed to build CORS layer")));

  let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
  let listener = TcpListener::bind(addr)
    .await
    .expect("Failed to bind to address");

  serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server");
}

fn routes() -> Router {
  Router::new().nest("/test", test::router())
}

collect_state!(test, logging);

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
