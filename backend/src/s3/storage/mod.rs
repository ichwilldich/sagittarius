use std::ops::Deref;
use std::path::{self, Path};
use std::sync::Arc;
use std::{io::Result, path::PathBuf};

use axum::Extension;
use centaurus::FromReqExtension;
use clap::ValueEnum;
use tokio::fs;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::config::Config;
use crate::macros::DualRouterExt;
use crate::router_extension;
use crate::s3::BUCKET_DIR;
use crate::s3::interface::S3Interface;

pub mod no_raid;

#[async_trait::async_trait]
pub trait Storage {
  async fn create_dir(&self, path: &Path) -> Result<()>;
  async fn delete_dir(&self, path: &Path) -> Result<()>;
  async fn list_dir(&self, path: &Path) -> Result<Vec<String>>;
  async fn stream_write_file(
    &self,
    path: &Path,
    reader: &mut (dyn AsyncRead + Unpin + Send),
  ) -> Result<()>;
  async fn stream_read_file(&self, path: &Path) -> Result<Box<dyn AsyncRead + Unpin + Send>>;
  async fn delete_file(&self, path: &Path) -> Result<()>;

  async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()> {
    self.stream_write_file(path, &mut &data[..]).await
  }

  async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
    let mut reader = self.stream_read_file(path).await?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    Ok(buf)
  }
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Debug)]
pub enum StorageType {
  NoRaid,
}

impl StorageType {
  pub async fn storage(&self, base_path: PathBuf) -> Result<S3Interface> {
    let base_path = path::absolute(base_path)?;
    if !base_path.exists() {
      fs::create_dir_all(&base_path).await?;
    }
    let bucket_path = base_path.join(BUCKET_DIR);
    if !bucket_path.exists() {
      fs::create_dir_all(&bucket_path).await?;
    }

    Ok(S3Interface::new(match self {
      StorageType::NoRaid => no_raid::NoRaid::new(base_path),
    }))
  }
}
