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
}

#[cfg(test)]
mod test {
  use super::*;
  use clap::CommandFactory;

  #[test]
  fn verify_config() {
    Config::command().debug_assert();
  }

  #[test]
  fn storage_type() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("STORAGE_TYPE", "no-raid");
    }
    // it fails when doing Config::parse() because there is some "--exact" arg
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.storage_type, StorageType::NoRaid);
  }

  #[test]
  fn storage_path() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
    }
    // it fails when doing Config::parse() because there is some "--exact" arg
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.storage_path, PathBuf::from("/tmp/s3"));
  }

  #[test]
  fn s3_port() {
    unsafe {
      std::env::set_var("STORAGE_PATH", "/tmp/s3");
      std::env::set_var("S3_PORT", "9000");
    }
    let cfg = Config::parse_from([""]);
    assert_eq!(cfg.s3_port, 9000);
  }
}
