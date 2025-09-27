use std::path::PathBuf;

use clap::Parser;
use ichwilldich_lib::{FromReqExtension, config::BaseConfig};

use crate::s3::storage::StorageType;

#[derive(Parser, Clone, FromReqExtension)]
pub struct Config {
  #[command(flatten)]
  pub base: BaseConfig,

  // storage
  #[clap(short, long, env, default_value = "no-raid")]
  pub storage_type: StorageType,
  #[clap(short = 'p', long, env)]
  pub storage_path: PathBuf,

  // s3
  #[clap(long, env, default_value = "9000")]
  pub s3_port: u16,

  // database
  #[clap(long, env)]
  pub db_url: String,
  #[clap(long, env, default_value = "1024")]
  pub database_max_connections: u32,
  #[clap(long, env, default_value = "1")]
  pub database_min_connections: u32,
  #[clap(long, env, default_value = "5")]
  pub database_connect_timeout: u64,
  #[clap(long, env, default_value = "false")]
  pub database_logging: bool,

  // jwt
  #[clap(long, env, default_value = "my_iss")]
  pub jwt_iss: String,
  #[clap(long, env, default_value = "604800")]
  pub jwt_exp: i64,

  // auth
  #[clap(long, env, default_value = "_my_pepper____123")]
  pub auth_pepper: String,
}

#[cfg(test)]
mod test {
  use super::*;
  use clap::CommandFactory;

  #[test]
  fn test_verify_config() {
    Config::command().debug_assert();
  }

  #[test]
  fn test_storage_type() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("STORAGE_TYPE", "no-raid");
    }
    // it fails when doing Config::parse() because there is some "--exact" arg
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.storage_type, StorageType::NoRaid);
  }

  #[test]
  fn test_storage_path() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
    }
    // it fails when doing Config::parse() because there is some "--exact" arg
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.storage_path, PathBuf::from("/tmp/s3"));
  }

  #[test]
  fn test_s3_port() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("S3_PORT", "9000");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.s3_port, 9000);
  }

  #[test]
  fn test_db_url() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.db_url, "postgresql://test:test@localhost:5432/test");
  }

  #[test]
  fn test_database_max_connections() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("DATABASE_MAX_CONNECTIONS", "1024");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.database_max_connections, 1024);
  }

  #[test]
  fn test_database_min_connections() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("DATABASE_MIN_CONNECTIONS", "1");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.database_min_connections, 1);
  }

  #[test]
  fn test_database_connect_timeout() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("DATABASE_CONNECT_TIMEOUT", "5");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.database_connect_timeout, 5);
  }

  #[test]
  fn test_database_logging() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("DATABASE_LOGGING", "false");
      std::env::set_var("DATABASE_MIN_CONNECTIONS", "1");
    }
    let cfg = Config::parse_from([""]);
    assert!(!cfg.database_logging);
  }

  #[test]
  fn test_jwt_iss() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("JWT_ISS", "my_iss");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.jwt_iss, "my_iss");
  }

  #[test]
  fn test_jwt_exp() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("DB_URL", "postgresql://test:test@localhost:5432/test");
      std::env::set_var("JWT_EXP", "604800");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.jwt_exp, 604800);
  }
}
