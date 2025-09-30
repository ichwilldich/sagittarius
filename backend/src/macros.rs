use std::convert::Infallible;

use axum::{extract::Request, response::IntoResponse, routing::Route, Router};
use tower::{Layer, Service};

#[macro_export]
macro_rules! router_extension {
  (async fn $name:ident($($arg:tt)*) -> Self { $($body:tt)* }) => {
    centaurus::router_extension!((axum::Router, axum::Router), async fn $name($($arg)*) -> Self { $($body)* });
  };
}

pub trait DualRouterExt<S>
where
  S: Clone + Send + Sync + 'static,
{
  fn layer<L>(self, layer: L) -> Self
  where
    L: Layer<Route> + Clone + Send + Sync + 'static,
    L::Service: Service<Request> + Clone + Send + Sync + 'static,
    <L::Service as Service<Request>>::Response: IntoResponse + 'static,
    <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
    <L::Service as Service<Request>>::Future: Send + 'static;
}

impl<S> DualRouterExt<S> for (Router<S>, Router<S>)
where
  S: Clone + Send + Sync + 'static,
{
  fn layer<L>(self, layer: L) -> Self
  where
    L: Layer<Route> + Clone + Send + Sync + 'static,
    L::Service: Service<Request> + Clone + Send + Sync + 'static,
    <L::Service as Service<Request>>::Response: IntoResponse + 'static,
    <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
    <L::Service as Service<Request>>::Future: Send + 'static,
  {
    (self.0.layer(layer.clone()), self.1.layer(layer))
  }
}
