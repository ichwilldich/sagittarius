use std::{
  io::{Error, ErrorKind, Result},
  path::{Path, PathBuf},
};

use tokio::{fs, io};

use crate::s3::storage::Storage;

pub struct NoRaid {
  base_path: PathBuf,
}

impl NoRaid {
  pub async fn new(base_path: PathBuf) -> Result<Self> {
    let base_path = fs::canonicalize(base_path).await?;
    Ok(Self { base_path })
  }

  async fn full_path(&self, path: &Path) -> Result<PathBuf> {
    let path = fs::canonicalize(self.base_path.join(path)).await?;

    if !path.starts_with(&self.base_path) {
      return Err(Error::new(
        ErrorKind::PermissionDenied,
        "Access outside of base path is not allowed",
      ));
    }

    Ok(path)
  }
}

impl Storage for NoRaid {
  async fn create_dir(&self, path: &Path) -> Result<()> {
    let full_path = self.full_path(path).await?;
    fs::create_dir_all(full_path).await
  }

  async fn delete_dir(&self, path: &Path) -> Result<()> {
    let full_path = self.full_path(path).await?;
    fs::remove_dir_all(full_path).await
  }

  async fn list_dir(&self, path: &Path) -> Result<Vec<String>> {
    let full_path = self.full_path(path).await?;
    let mut entries = fs::read_dir(full_path).await?;
    let mut names = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
      if let Some(name) = entry.file_name().to_str() {
        names.push(name.to_string());
      }
    }
    Ok(names)
  }

  async fn stream_write_file<R: tokio::io::AsyncRead + Unpin + Send>(
    &self,
    path: &Path,
    mut reader: R,
  ) -> Result<()> {
    let full_path = self.full_path(path).await?;
    let mut file = fs::File::create(full_path).await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
  }

  async fn stream_read_file(
    &self,
    path: &Path,
  ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send>> {
    let full_path = self.full_path(path).await?;
    let file = fs::File::open(full_path).await?;
    Ok(Box::new(file))
  }

  async fn delete_file(&self, path: &Path) -> Result<()> {
    let full_path = self.full_path(path).await?;
    fs::remove_file(full_path).await
  }
}
