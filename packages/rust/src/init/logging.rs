use http::Request;
use tower_http::trace::TraceLayer;

use crate::router_extension;

router_extension!(
  async fn logging(self) -> Self {
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
