use axum::{Router, routing::post};
use axum_extra::extract::CookieJar;
use chrono::DateTime;
use eyre::ContextCompat;
use ichwilldich_lib::error::Result;
use time::Duration;

use crate::{
  auth::{
    jwt_auth::{COOKIE_NAME, JwtAuth},
    jwt_state::{JwtInvalidState, JwtState},
    res::TokenRes,
  },
  db::Connection,
};

pub fn router() -> Router {
  Router::new().route("/logout", post(logout))
}

async fn logout(
  auth: JwtAuth,
  db: Connection,
  mut cookies: CookieJar,
  state: JwtInvalidState,
  jwt: JwtState,
) -> Result<(CookieJar, TokenRes)> {
  let mut reset_cookie = jwt.create_cookie(COOKIE_NAME.to_string());
  reset_cookie.set_max_age(Duration::seconds(0));

  let cookie = cookies.get(COOKIE_NAME).context("Missing auth cookie")?;
  let mut count = state.count.lock().await;

  db.invalid_jwt()
    .invalidate_jwt(
      cookie.value().to_string(),
      DateTime::from_timestamp(auth.exp, 0).context("invalid timestamp")?,
      &mut count,
    )
    .await?;

  cookies = cookies.remove(reset_cookie);

  Ok((cookies, TokenRes))
}
