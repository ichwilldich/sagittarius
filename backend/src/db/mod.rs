use std::{ops::Deref, time::Duration};

use centaurus::FromReqExtension;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

mod invalid_jwt;
mod key;
mod user;

pub async fn init_db(config: &crate::config::Config) -> Connection {
  let mut options = ConnectOptions::new(&config.db_url);
  options
    .max_connections(config.database_max_connections)
    .min_connections(config.database_min_connections)
    .connect_timeout(Duration::from_secs(config.database_connect_timeout))
    .sqlx_logging(config.database_logging);

  let conn = Database::connect(options)
    .await
    .expect("Failed to connect to database");
  migration::Migrator::up(&conn, None)
    .await
    .expect("Failed to run database migrations");

  Connection(conn)
}

#[derive(FromReqExtension, Clone)]
pub struct Connection(DatabaseConnection);

impl Deref for Connection {
  type Target = DatabaseConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Connection {
  pub fn key(&self) -> key::KeyTable<'_> {
    key::KeyTable::new(&self.0)
  }

  pub fn user(&self) -> user::UserTable<'_> {
    user::UserTable::new(&self.0)
  }

  pub fn invalid_jwt(&self) -> invalid_jwt::InvalidJwtTable<'_> {
    invalid_jwt::InvalidJwtTable::new(&self.0)
  }
}
