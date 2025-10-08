use axum::{Router, routing::get};
use centaurus::error::Result;

use crate::{
  auth::jwt_auth::{InternalAuth, JwtAuth},
  db::Connection,
};

pub fn router() -> Router {
  Router::new()
    .route("/example_internal", get(example_internal))
    .route("/example_all", get(example_all))
}

async fn example_internal(auth: JwtAuth<InternalAuth>, db: Connection) -> Result<String> {
  let user = db.user().get_user(auth.user_id).await?;
  Ok(format!("Hello, {}!", user.name))
}

async fn example_all(auth: JwtAuth) -> Result<String> {
  Ok(format!("Hello, {}!", auth.user_id))
}
