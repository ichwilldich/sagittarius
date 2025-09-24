use axum::{
  RequestPartsExt,
  extract::{FromRequest, Request},
};
use axum_extra::{
  TypedHeader,
  headers::{ContentType, Mime},
};
use http::Method;
use ichwilldich_lib::error::ErrorReport;
use tracing::instrument;

use crate::s3::auth::{header::header_auth, multipart::multipart_auth, query::query_auth};

mod credential;
mod header;
mod multipart;
mod query;
mod sig_v4;

/// TODO: body handling, maybe body caching, credential lookup, expiration check, region check
/// https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html
pub struct S3Auth {
  region: String,
  access_key: String,
}

const SECRET: &str = "secret";

impl<S: Sync + Send> FromRequest<S> for S3Auth {
  type Rejection = ErrorReport;

  #[instrument(skip(_state))]
  async fn from_request(req: Request, _state: &S) -> std::result::Result<Self, Self::Rejection> {
    let (mut req, body) = req.into_parts();
    if req.method == Method::POST
      && let Ok(TypedHeader(mime)) = req.extract::<TypedHeader<ContentType>>().await
    {
      let mime: Mime = mime.into();
      if mime.type_() == mime::MULTIPART && mime.subtype() == mime::FORM_DATA {
        return multipart_auth(Request::from_parts(req, body)).await;
      }
    }

    let query: Vec<(String, String)> =
      serde_urlencoded::from_str(req.uri.query().unwrap_or("")).unwrap_or_default();
    if query.iter().any(|(k, _)| k == "X-Amz-Signature") {
      return query_auth(Request::from_parts(req, body), &query).await;
    }

    header_auth(Request::from_parts(req, body)).await
  }
}
