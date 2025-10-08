use std::sync::Arc;

use axum_extra::extract::cookie::{Cookie, SameSite};
use centaurus::{FromReqExtension, error::Result};
use chrono::{Duration, Utc};
use eyre::ContextCompat;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rsa::{
  RsaPrivateKey, RsaPublicKey,
  pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
  auth::jwt_auth::{AuthSource, COOKIE_NAME},
  config::EnvConfig,
  db::Connection,
};

const JWT_KEY_NAME: &str = "jwt";

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
  pub exp: i64,
  pub iss: String,
  pub sub: String,
  pub r#type: AuthType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AuthType {
  Oidc,
  Internal,
}

#[derive(FromReqExtension, Clone)]
pub struct JwtState {
  header: Header,
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
  validation: Validation,
  iss: String,
  exp: i64,
}

impl JwtState {
  pub fn create_token<'c, T: AuthSource>(
    &self,
    uuid: T::UserID,
    r#type: AuthType,
  ) -> Result<Cookie<'c>> {
    let exp = Utc::now()
      .checked_add_signed(Duration::seconds(self.exp))
      .context("invalid timestamp")?
      .timestamp();

    let claims = JwtClaims {
      exp,
      iss: self.iss.clone(),
      sub: uuid.to_string(),
      r#type,
    };

    let token = encode(&self.header, &claims, &self.encoding_key)?;

    Ok(self.create_cookie(COOKIE_NAME, token))
  }

  pub fn create_cookie<'c>(&self, key: &'c str, token: String) -> Cookie<'c> {
    Cookie::build((key, token))
      .http_only(true)
      .max_age(time::Duration::seconds(self.exp))
      .same_site(SameSite::Lax)
      .secure(true)
      .path("/")
      .build()
  }

  pub fn validate_token(&self, token: &str) -> Result<JwtClaims> {
    let token_data = decode::<JwtClaims>(token, &self.decoding_key, &self.validation)?;

    Ok(token_data.claims)
  }

  pub async fn init(config: &EnvConfig, db: &Connection) -> Self {
    let (key, kid) = if let Ok(key) = db.key().get_key_by_name(JWT_KEY_NAME.into()).await {
      (key.private_key, key.id.to_string())
    } else {
      let mut rng = OsRng {};
      let private_key = RsaPrivateKey::new(&mut rng, 4096).expect("Failed to create Rsa key");
      let key = private_key
        .to_pkcs1_pem(LineEnding::CRLF)
        .expect("Failed to export private key")
        .to_string();

      let uuid = Uuid::new_v4();

      db.key()
        .create_key(JWT_KEY_NAME.into(), key.clone(), uuid)
        .await
        .expect("Failed to save key");

      (key, uuid.to_string())
    };

    let private_key = RsaPrivateKey::from_pkcs1_pem(&key).expect("Failed to load public key");
    let public_key = RsaPublicKey::from(private_key);
    let public_key_pem = public_key
      .to_pkcs1_pem(LineEnding::CRLF)
      .expect("Failed to export public key");

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.clone());

    let encoding_key =
      EncodingKey::from_rsa_pem(key.as_bytes()).expect("Failed to create encoding key");
    let decoding_key =
      DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).expect("Failed to create decoding key");

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_aud = false;

    Self {
      header,
      encoding_key,
      decoding_key,
      validation,
      exp: config.auth.jwt_exp,
      iss: config.auth.jwt_iss.clone(),
    }
  }
}

#[derive(FromReqExtension, Clone, Default)]
pub struct JwtInvalidState {
  pub count: Arc<Mutex<i32>>,
}
