use axum::{Json, Router, routing::get};
use axum_extra::extract::CookieJar;
use time::Duration;

use crate::auth::{jwt_auth::JwtAuth, jwt_state::JwtState};

pub fn router() -> Router {
  Router::new().route("/test_token", get(test_token))
}

async fn test_token(
  auth: Option<JwtAuth>,
  mut cookies: CookieJar,
  jwt: JwtState,
) -> (CookieJar, Json<bool>) {
  if auth.is_none() {
    let mut reset_cookie = jwt.create_cookie(String::new());
    reset_cookie.set_max_age(Some(Duration::seconds(0)));
    cookies = cookies.remove(reset_cookie);

    (cookies, Json(false))
  } else {
    (cookies, Json(true))
  }
}
