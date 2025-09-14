use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
  #[error("BadRequest")]
  BadRequest,
  #[error("Unauthorized")]
  Unauthorized,
  #[allow(clippy::enum_variant_names)]
  #[error("InternalServerError")]
  InternalServerError,
  #[error("Conflict")]
  Conflict,
  #[error("Gone")]
  Gone,
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    tracing::error!("{:?}", &self);
    match self {
      Self::BadRequest => StatusCode::BAD_REQUEST.into_response(),
      Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
      Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
      Self::Conflict => StatusCode::CONFLICT.into_response(),
      Self::Gone => StatusCode::GONE.into_response(),
      _ => StatusCode::BAD_REQUEST.into_response(),
    }
  }
}
