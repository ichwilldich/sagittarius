use std::{marker::PhantomData, str::FromStr};

use axum::{
  RequestPartsExt,
  extract::{FromRequestParts, OptionalFromRequestParts},
};
use axum_extra::{
  TypedHeader,
  extract::CookieJar,
  headers::{Authorization, authorization::Bearer},
};
use centaurus::{bail, error::ErrorReport};
use http::request::Parts;
use uuid::Uuid;

use crate::{
  auth::jwt_state::{AuthType, JwtState},
  db::Connection,
};

pub const COOKIE_NAME: &str = "auth_token";

pub trait AuthSource {
  type UserID: FromStr + ToString;

  fn internal() -> bool {
    false
  }
}

pub struct AllAuth;
impl AuthSource for AllAuth {
  type UserID = String;
}

pub struct InternalAuth;
impl AuthSource for InternalAuth {
  type UserID = Uuid;

  fn internal() -> bool {
    true
  }
}

pub struct JwtAuth<T: AuthSource = AllAuth> {
  pub user_id: T::UserID,
  pub exp: i64,
  _m: PhantomData<T>,
}

impl<S: Sync, T: AuthSource> FromRequestParts<S> for JwtAuth<T> {
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
    let Ok(user_id) = T::UserID::from_str(&claims.sub) else {
      bail!(UNAUTHORIZED, "invalid user id in token");
    };

    if T::internal() && claims.r#type != AuthType::Internal {
      bail!(UNAUTHORIZED, "token is not for internal use");
    }

    Ok(JwtAuth {
      user_id,
      exp: claims.exp,
      _m: PhantomData,
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
