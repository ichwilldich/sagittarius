use axum::{
  RequestPartsExt,
  extract::{FromRequestParts, OptionalFromRequestParts},
};
use axum_extra::{
  TypedHeader,
  extract::CookieJar,
  headers::{Authorization, authorization::Bearer},
};
use http::request::Parts;
use ichwilldich_lib::{bail, error::ErrorReport};
use uuid::Uuid;

use crate::{auth::jwt_state::JwtState, db::Connection};

pub const COOKIE_NAME: &str = "auth_token";

pub struct JwtAuth {
  pub user_id: Uuid,
  pub exp: i64,
}

impl<S: Sync> FromRequestParts<S> for JwtAuth {
  type Rejection = ErrorReport;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let bearer = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;

    let token = match bearer {
      Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
      Err(_) => {
        let Ok(cookie) = parts.extract::<CookieJar>().await;
        match cookie.get(COOKIE_NAME) {
          Some(c) => c.value().to_string(),
          None => bail!("no auth token found"),
        }
      }
    };

    let Ok(state) = parts.extract::<JwtState>().await;
    let Ok(db) = parts.extract::<Connection>().await;

    let Ok(valid) = db.invalid_jwt().is_token_valid(&token).await else {
      bail!("failed to validate jwt");
    };
    if !valid {
      bail!(UNAUTHORIZED, "token is invalidated");
    }

    let Ok(claims) = state.validate_token(&token) else {
      bail!(UNAUTHORIZED, "invalid token");
    };

    Ok(JwtAuth {
      user_id: claims.sub,
      exp: claims.exp,
    })
  }
}

impl<S: Sync> OptionalFromRequestParts<S> for JwtAuth {
  type Rejection = ErrorReport;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Option<Self>, Self::Rejection> {
    match <Self as FromRequestParts<S>>::from_request_parts(parts, state).await {
      Ok(auth) => Ok(Some(auth)),
      Err(_) => Ok(None),
    }
  }
}
