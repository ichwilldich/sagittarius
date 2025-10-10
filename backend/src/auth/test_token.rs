use axum::{Json, Router, routing::get};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::auth::jwt_auth::{COOKIE_NAME, JwtAuth};

pub fn router() -> Router {
  Router::new().route("/test_token", get(test_token))
}

async fn test_token(auth: Option<JwtAuth>, mut cookies: CookieJar) -> (CookieJar, Json<bool>) {
  if auth.is_none() {
    cookies = cookies.remove(Cookie::from(COOKIE_NAME));

    (cookies, Json(false))
  } else {
    (cookies, Json(true))
  }
}
