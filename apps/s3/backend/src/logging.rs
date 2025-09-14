use http::Request;
use tower_http::trace::TraceLayer;

use crate::{config::Config, state_trait};

state_trait!(
  async fn logging(self, _config: &Config) -> Self {
    self.layer(
      TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &tracing::Span| {
          tracing::info!("Received request: {}", request.uri());
        })
        .on_response(
          |response: &http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
            tracing::info!(
              "Response sent with status: {} in {:?}",
              response.status(),
              latency
            );
          },
        ),
    )
  }
);
