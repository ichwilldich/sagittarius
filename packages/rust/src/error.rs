use std::{fmt::Debug, num::ParseIntError};

use axum::{
  extract::{
    multipart::{MultipartError, MultipartRejection},
    rejection::BytesRejection,
  },
  response::{IntoResponse, Response},
};
use axum_extra::typed_header::TypedHeaderRejection;
use hmac::digest::InvalidLength;
use http::StatusCode;

pub type Result<T> = std::result::Result<T, ErrorReport>;

#[macro_export]
macro_rules! anyhow {
  ($status:ident, $msg:literal) => {
    $crate::error::ErrorReport::new($crate::eyre::eyre!($msg), $crate::http::StatusCode::$status)
  };
  ($msg:literal) => {
    $crate::anyhow!(BAD_REQUEST, $msg)
  };
  ($status:ident, $err:expr) => {
    $crate::error::ErrorReport::new($crate::eyre::eyre!($err), $crate::http::StatusCode::$status)
  };
  ($err:expr) => {
    $crate::anyhow!(BAD_REQUEST, $err)
  };
  ($status:ident, $fmt:expr, $($arg:tt)*) => {
    $crate::error::ErrorReport::new($crate::eyre::eyre!($fmt, $($arg)*), $crate::http::StatusCode::$status)
  };
  ($fmt:expr, $($arg:tt)*) => {
    $crate::anyhow!(BAD_REQUEST, $fmt, $($arg)*)
  };
}

#[macro_export]
macro_rules! bail {
  ($status:ident, $msg:literal) => {
    return $crate::private::Err($crate::anyhow!($status, $msg));
  };
  ($msg:literal) => {
    return $crate::private::Err($crate::anyhow!($msg));
  };
  ($status:ident, $err:expr) => {
    return $crate::private::Err($crate::anyhow!($status, $err));
  };
  ($err:expr) => {
    return $crate::private::Err($crate::anyhow!($err));
  };
  ($status:ident, $fmt:expr, $($arg:tt)*) => {
    return $crate::private::Err($crate::anyhow!($status, $fmt, $($arg)*));
  };
  ($fmt:expr, $($arg:tt)*) => {
    return $crate::private::Err($crate::anyhow!($fmt, $($arg)*));
  };
}

#[macro_export]
macro_rules! impl_from_error {
  ($error:ty, $status:expr) => {
    impl From<$error> for ErrorReport {
      #[track_caller]
      fn from(value: $error) -> Self {
        Self {
          error: eyre::Report::new(value),
          status: $status,
        }
      }
    }
  };
}

pub struct ErrorReport {
  error: eyre::Report,
  status: StatusCode,
}

impl ErrorReport {
  pub fn new(error: eyre::Report, status: StatusCode) -> Self {
    Self { error, status }
  }
}

impl_from_error!(std::io::Error, StatusCode::INTERNAL_SERVER_ERROR);
impl_from_error!(http::header::InvalidHeaderValue, StatusCode::BAD_REQUEST);
impl_from_error!(TypedHeaderRejection, StatusCode::BAD_REQUEST);
impl_from_error!(BytesRejection, StatusCode::BAD_REQUEST);
impl_from_error!(InvalidLength, StatusCode::BAD_REQUEST);
impl_from_error!(MultipartRejection, StatusCode::BAD_REQUEST);
impl_from_error!(MultipartError, StatusCode::BAD_REQUEST);
impl_from_error!(chrono::ParseError, StatusCode::BAD_REQUEST);
impl_from_error!(ParseIntError, StatusCode::BAD_REQUEST);
impl_from_error!(serde_xml_rs::Error, StatusCode::BAD_REQUEST);
impl_from_error!(jsonwebtoken::errors::Error, StatusCode::BAD_REQUEST);
impl_from_error!(sea_orm::DbErr, StatusCode::INTERNAL_SERVER_ERROR);
impl_from_error!(base64::DecodeError, StatusCode::BAD_REQUEST);
impl_from_error!(rsa::Error, StatusCode::BAD_REQUEST);
impl_from_error!(password_hash::Error, StatusCode::BAD_REQUEST);

impl IntoResponse for ErrorReport {
  fn into_response(self) -> Response {
    tracing::error!("{:?}", self.error);
    self.status.into_response()
  }
}

impl Debug for ErrorReport {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <eyre::Report as Debug>::fmt(&self.error, f)
  }
}

impl From<eyre::Report> for ErrorReport {
  fn from(value: eyre::Report) -> Self {
    Self {
      error: value,
      status: StatusCode::BAD_REQUEST,
    }
  }
}
