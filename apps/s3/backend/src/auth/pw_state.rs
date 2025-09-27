use argon2::{
  Argon2,
  password_hash::{PasswordHasher, SaltString},
};
use base64::{
  Engine,
  prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD},
};
use ichwilldich_lib::{FromReqExtension, error::Result};
use rsa::{
  Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
  pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
};
use tracing::info;
use uuid::Uuid;

use crate::{config::Config, db::Connection};

const PW_KEY: &str = "password";

#[derive(FromReqExtension, Clone)]
pub struct PasswordState {
  key: RsaPrivateKey,
  pub pub_key: String,
  pepper: Vec<u8>,
}

impl PasswordState {
  pub fn pw_hash(&self, salt: &str, password: &str) -> Result<String> {
    let bytes = BASE64_STANDARD.decode(password)?;
    let pw_bytes = self.key.decrypt(Pkcs1v15Encrypt, &bytes)?;
    let password = String::from_utf8_lossy(&pw_bytes).to_string();

    self.pw_hash_raw(salt, &password)
  }

  pub fn pw_hash_raw(&self, salt: &str, password: &str) -> Result<String> {
    let mut salt = BASE64_STANDARD_NO_PAD.decode(salt)?;
    salt.extend_from_slice(&self.pepper);
    let salt_string = SaltString::encode_b64(&salt)?;

    let argon2 = Argon2::default();
    let password_hash = argon2
      .hash_password(password.as_bytes(), salt_string.as_salt())?
      .to_string();

    Ok(password_hash)
  }

  pub async fn init(config: &Config, db: &Connection) -> Self {
    let key = if let Ok(key) = db.key().get_key_by_name(PW_KEY.into()).await {
      RsaPrivateKey::from_pkcs1_pem(&key.private_key).expect("Failed to parse private password key")
    } else {
      let mut rng = OsRng {};
      let private_key = RsaPrivateKey::new(&mut rng, 4096).expect("Failed to create Rsa key");
      let key = private_key
        .to_pkcs1_pem(LineEnding::CRLF)
        .expect("Failed to export private key")
        .to_string();

      db.key()
        .create_key("password".into(), key.clone(), Uuid::new_v4())
        .await
        .expect("Failed to save key");

      private_key
    };

    let pub_key = RsaPublicKey::from(&key)
      .to_pkcs1_pem(LineEnding::CRLF)
      .expect("Failed to export Rsa Public Key");

    let pepper = config.auth_pepper.as_bytes().to_vec();
    if pepper.len() > 32 {
      panic!("Pepper is longer than 32 characters");
    }

    let state = Self {
      key,
      pub_key,
      pepper,
    };

    // initial user setup
    let user_count = db
      .user()
      .list_users()
      .await
      .expect("Failed to list users")
      .len();
    if user_count == 0 || config.overwrite_initial_user {
      let salt = SaltString::generate(OsRng {}).to_string();
      let password = state
        .pw_hash_raw(&salt, &config.initial_user_password)
        .expect("Failed to hash initial password");

      let user = entity::user::Model {
        id: Uuid::new_v4(),
        name: config.initial_user_username.clone(),
        password,
        salt,
      };

      db.user()
        .create_user(user)
        .await
        .expect("Failed to create initial user");

      info!("Initial user '{}' created", config.initial_user_username);
    } else {
      info!("Users already exist, skipping initial user creation");
    }

    state
  }
}
