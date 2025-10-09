use figment::{
  Figment,
  providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SavedConfig {
  #[serde(default)]
  pub oidc: SSOConfig,
}

impl SavedConfig {
  pub fn parse() -> Self {
    let config = Figment::new()
      .merge(Serialized::defaults(Self::default()))
      .merge(Env::raw().global());

    config.extract().expect("failed to load configuration")
  }
}

#[derive(Clone)]
pub struct MergedConfig {
  pub oidc: MergedSSOConfig,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SSOConfig {
  sso_instant_redirect: Option<bool>,

  oidc_client_id: Option<String>,
  oidc_client_secret: Option<String>,
  oidc_url: Option<Url>,
  oidc_scope: Option<String>,
}

#[derive(Clone)]
pub struct MergedSSOConfig {
  pub sso_instant_redirect: ConfigValue<bool>,
  pub oidc_client_id: ConfigValue<String>,
  pub oidc_client_secret: ConfigValue<String>,
  pub oidc_url: ConfigValue<Url>,
  pub oidc_scope: ConfigValue<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum ConfigValue<T> {
  Env(T),
  UI(T),
  None,
}

impl<T> ConfigValue<T> {
  pub fn value(&self) -> Option<&T> {
    match self {
      ConfigValue::Env(v) => Some(v),
      ConfigValue::UI(v) => Some(v),
      ConfigValue::None => None,
    }
  }

  fn from_value(env: Option<T>, ui: Option<T>) -> Self {
    if let Some(v) = env {
      ConfigValue::Env(v)
    } else if let Some(v) = ui {
      ConfigValue::UI(v)
    } else {
      ConfigValue::None
    }
  }
}

impl SavedConfig {
  pub(crate) fn merge(self, ui: Self) -> MergedConfig {
    MergedConfig {
      oidc: self.oidc.merge(ui.oidc),
    }
  }
}

impl MergedConfig {
  pub(super) fn to_ui(&self) -> SavedConfig {
    SavedConfig {
      oidc: self.oidc.to_ui(),
    }
  }
}

impl SSOConfig {
  pub(crate) fn merge(self, ui: Self) -> MergedSSOConfig {
    MergedSSOConfig {
      sso_instant_redirect: ConfigValue::from_value(
        ui.sso_instant_redirect,
        self.sso_instant_redirect,
      ),
      oidc_client_id: ConfigValue::from_value(ui.oidc_client_id, self.oidc_client_id),
      oidc_client_secret: ConfigValue::from_value(ui.oidc_client_secret, self.oidc_client_secret),
      oidc_url: ConfigValue::from_value(ui.oidc_url, self.oidc_url),
      oidc_scope: ConfigValue::from_value(ui.oidc_scope, self.oidc_scope),
    }
  }
}

impl MergedSSOConfig {
  pub(super) fn to_ui(&self) -> SSOConfig {
    SSOConfig {
      sso_instant_redirect: self.sso_instant_redirect.value().cloned(),
      oidc_client_id: self.oidc_client_id.value().cloned(),
      oidc_client_secret: self.oidc_client_secret.value().cloned(),
      oidc_url: self.oidc_url.value().cloned(),
      oidc_scope: self.oidc_scope.value().cloned(),
    }
  }
}
