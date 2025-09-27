use axum::{
  body::Body,
  response::{IntoResponse, Response},
};
use http::header::{CACHE_CONTROL, PRAGMA};

pub struct TokenRes;

impl IntoResponse for TokenRes {
  fn into_response(self) -> Response {
    Response::builder()
      .header(CACHE_CONTROL, "no-store")
      .header(PRAGMA, "no-cache")
      .body(Body::empty())
      .unwrap()
  }
}
