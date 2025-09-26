use std::{ops::Deref, time::Duration};

use axum::Extension;
use ichwilldich_lib::FromReqExtension;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{macros::DualRouterExt, router_extension};

router_extension!(
  async fn db(self, config: &crate::config::Config) -> Self {
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

    self.layer(Extension(Connection(conn)))
  }
);

#[derive(FromReqExtension, Clone)]
pub struct Connection(DatabaseConnection);

impl Deref for Connection {
  type Target = DatabaseConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Connection {
  pub fn tables(&self) -> Tables<'_> {
    Tables { conn: &self.0 }
  }
}

pub struct Tables<'db> {
  conn: &'db DatabaseConnection,
}
