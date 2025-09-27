use axum::{Router, routing::get};
use ichwilldich_lib::error::Result;

use crate::{auth::jwt_auth::JwtAuth, db::Connection};

pub fn router() -> Router {
  Router::new().route("/example", get(example))
}

async fn example(auth: JwtAuth, db: Connection) -> Result<String> {
  let user = db.user().get_user(auth.user_id).await?;
  Ok(format!("Hello, {}!", user.name))
}
