use clap::Parser;
use ichwilldich_lib::{FromReqExtension, config::BaseConfig};

#[derive(Parser, Clone, FromReqExtension)]
pub struct Config {
  #[command(flatten)]
  pub base: BaseConfig,
}
