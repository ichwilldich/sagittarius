use axum::{Extension, Router};

use crate::{auth::jwt_state::JwtState, db::Connection, macros::DualRouterExt, router_extension};

mod jwt_auth;
mod jwt_state;
mod test_token;

pub fn router() -> Router {
  test_token::router()
}

router_extension!(
  async fn auth(self, config: &crate::config::Config, db: &Connection) -> Self {
    self.layer(Extension(JwtState::init(config, db).await))
  }
);
