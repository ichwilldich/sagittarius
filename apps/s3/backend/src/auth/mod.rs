use axum::{Extension, Router};

use crate::{
  auth::{
    jwt_state::{JwtInvalidState, JwtState},
    pw_state::PasswordState,
  },
  db::Connection,
  macros::DualRouterExt,
  router_extension,
};

pub mod jwt_auth;
mod jwt_state;
mod login;
mod logout;
mod pw_state;
mod res;
mod test_token;

pub fn router() -> Router {
  test_token::router()
    .merge(login::router())
    .merge(logout::router())
}

router_extension!(
  async fn auth(self, config: &crate::config::Config, db: &Connection) -> Self {
    self
      .layer(Extension(JwtState::init(config, db).await))
      .layer(Extension(PasswordState::init(config, db).await))
      .layer(Extension(JwtInvalidState::default()))
  }
);
