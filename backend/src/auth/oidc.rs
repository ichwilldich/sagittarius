use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use axum::{
  Json, Router,
  extract::Query,
  response::{IntoResponse, Response},
  routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use centaurus::{FromReqExtension, bail, error::Result};
use http::{StatusCode, header::LOCATION};
use jsonwebtoken::{
  DecodingKey, Validation,
  jwk::{AlgorithmParameters, JwkSet},
};
use reqwest::{Client, multipart::Form, redirect::Policy};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

use crate::{
  auth::{
    jwt_auth::AllAuth,
    jwt_state::{AuthType, JwtState},
  },
  config::{AppConfig, EnvConfig},
};

pub const OIDC_STATE: &str = "oidc_state";

pub fn router() -> Router {
  Router::new()
    .route("/oidc_url", get(oidc_url))
    .route("/oidc_callback", get(oidc_callback))
}

#[derive(Clone, FromReqExtension)]
pub struct OidcState {
  config: Option<OidcConfig>,
}

#[derive(Clone)]
struct OidcConfig {
  state: Arc<Mutex<HashSet<Uuid>>>,
  nonce: Arc<Mutex<HashSet<Uuid>>>,
  issuer: String,
  authorization_endpoint: Url,
  token_endpoint: Url,
  userinfo_endpoint: Url,
  jwk_set: JwkSet,
  client_id: String,
  client_secret: String,
  client: Client,
}

#[derive(Deserialize, Debug)]
struct OidcConfiguration {
  issuer: String,
  authorization_endpoint: Url,
  token_endpoint: Url,
  userinfo_endpoint: Url,
  jwks_uri: Url,
}

impl OidcState {
  pub async fn new(app_config: &AppConfig) -> Result<Self> {
    let config = if let Some(url) = app_config.config.oidc.oidc_url.value()
      && let Some(client_id) = app_config.config.oidc.oidc_client_id.value()
      && let Some(client_secret) = app_config.config.oidc.oidc_client_secret.value()
    {
      let config: OidcConfiguration = reqwest::get(url.clone()).await?.json().await?;
      let jwk_set: JwkSet = reqwest::get(config.jwks_uri).await?.json().await?;

      let client = Client::builder().redirect(Policy::none()).build()?;

      Some(OidcConfig {
        state: Default::default(),
        nonce: Default::default(),
        issuer: config.issuer,
        authorization_endpoint: config.authorization_endpoint,
        token_endpoint: config.token_endpoint,
        userinfo_endpoint: config.userinfo_endpoint,
        jwk_set,
        client_id: client_id.clone(),
        client_secret: client_secret.clone(),
        client,
      })
    } else {
      None
    };

    Ok(Self { config })
  }
}

impl OidcConfig {
  async fn validate_jwk(&self, token: &str) -> Result<()> {
    let header = jsonwebtoken::decode_header(token)?;

    let Some(kid) = header.kid else {
      bail!(INTERNAL_SERVER_ERROR, "Missing kid in JWK header");
    };

    let Some(jwk) = self.jwk_set.find(&kid) else {
      bail!(INTERNAL_SERVER_ERROR, "JWK not found");
    };

    let decoding_key = match &jwk.algorithm {
      AlgorithmParameters::RSA(rsa) => DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?,
      _ => {
        bail!(INTERNAL_SERVER_ERROR, "Unsupported JWK algorithm");
      }
    };

    let validation = {
      let mut validation = Validation::new(header.alg);
      validation.set_audience(&[self.client_id.to_string()]);
      validation.set_issuer(&[&self.issuer]);
      validation.validate_exp = false;
      validation
    };

    let data = jsonwebtoken::decode::<HashMap<String, serde_json::Value>>(
      token,
      &decoding_key,
      &validation,
    )?;

    let Some(Some(Ok(nonce))) = data
      .claims
      .get("nonce")
      .map(|nonce| nonce.as_str().map(|nonce| nonce.parse()))
    else {
      bail!(INTERNAL_SERVER_ERROR, "Missing nonce in JWK claims");
    };
    if !self.nonce.lock().await.remove(&nonce) {
      bail!(INTERNAL_SERVER_ERROR, "Invalid nonce");
    }

    Ok(())
  }
}

#[derive(Serialize)]
struct OidcResponse {
  url: Option<String>,
}

async fn oidc_url(
  state: OidcState,
  jwt: JwtState,
  mut cookies: CookieJar,
) -> Result<(CookieJar, Json<OidcResponse>)> {
  let url = if let Some(config) = &state.config {
    let state = Uuid::new_v4();
    let nonce = Uuid::new_v4();

    let form = Form::new()
      .text("response_type", "code")
      .text("client_id", config.client_id.clone())
      .text("state", state.to_string())
      .text("nonce", nonce.to_string());

    let req = config
      .client
      .post(config.authorization_endpoint.clone())
      .multipart(form)
      .build()?;

    let res = config.client.execute(req).await?;

    if !res.status().is_redirection() {
      bail!(INTERNAL_SERVER_ERROR, "OIDC authorization request failed");
    }
    let Some(location) = res.headers().get(LOCATION).and_then(|h| h.to_str().ok()) else {
      bail!(
        INTERNAL_SERVER_ERROR,
        "OIDC authorization response missing location header"
      );
    };

    config.state.lock().await.insert(state);
    cookies = cookies.add(jwt.create_cookie(OIDC_STATE, state.to_string()));

    config.nonce.lock().await.insert(nonce);

    Some(location.to_string())
  } else {
    None
  };

  Ok((cookies, Json(OidcResponse { url })))
}

#[derive(Deserialize)]
struct OidcCallbackQuery {
  code: Option<String>,
  state: Uuid,
  error: Option<String>,
}

struct Redirect {
  location: Url,
}

impl IntoResponse for Redirect {
  fn into_response(self) -> Response {
    (StatusCode::FOUND, [(LOCATION, self.location.to_string())]).into_response()
  }
}

#[derive(Deserialize)]
struct TokenRes {
  id_token: String,
}

#[derive(Deserialize)]
pub struct AuthInfo {
  pub sub: String,
}

async fn oidc_callback(
  Query(OidcCallbackQuery { code, state, error }): Query<OidcCallbackQuery>,
  oidc_state: OidcState,
  mut cookies: CookieJar,
  config: EnvConfig,
  jwt: JwtState,
) -> Result<(CookieJar, Redirect)> {
  let (path, error) = if let Some(config) = &oidc_state.config {
    if !config.state.lock().await.remove(&state) {
      bail!(BAD_REQUEST, "Invalid OIDC state");
    }
    let Some(cookie) = cookies.get(OIDC_STATE) else {
      bail!(BAD_REQUEST, "Missing OIDC state cookie");
    };
    if cookie.value() != state.to_string() {
      bail!(BAD_REQUEST, "OIDC state mismatch");
    }
    cookies = cookies.remove(Cookie::from(OIDC_STATE));

    if let Some(error) = error {
      ("/login", Some(error))
    } else if let Some(code) = code {
      let form = Form::new()
        .text("grant_type", "authorization_code")
        .text("code", code);

      let req = config
        .client
        .post(config.token_endpoint.clone())
        .basic_auth(config.client_id.clone(), Some(config.client_secret.clone()))
        .multipart(form)
        .build()?;

      let res: TokenRes = config.client.execute(req).await?.json().await?;
      config.validate_jwk(&res.id_token).await?;

      let req = config
        .client
        .get(config.userinfo_endpoint.clone())
        .bearer_auth(res.id_token)
        .build()?;
      let res: AuthInfo = config.client.execute(req).await?.json().await?;

      cookies = cookies.add(jwt.create_token::<AllAuth>(res.sub, AuthType::Oidc)?);

      ("/", None)
    } else {
      ("/login", Some("missing_code".to_string()))
    }
  } else {
    ("/login", Some("oidc_not_configured".to_string()))
  };

  let mut url = config.base_url;
  url.set_path(path);
  url.set_query(error.map(|e| format!("error={e}")).as_deref());

  Ok((cookies, Redirect { location: url }))
}
