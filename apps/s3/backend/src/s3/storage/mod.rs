use std::io::Result;
use std::path::Path;

use tokio::io::{AsyncRead, AsyncReadExt};

pub mod no_raid;

pub trait Storage {
  async fn create_dir(&self, path: &Path) -> Result<()>;
  async fn delete_dir(&self, path: &Path) -> Result<()>;
  async fn list_dir(&self, path: &Path) -> Result<Vec<String>>;
  async fn stream_write_file<R: AsyncRead + Unpin + Send>(
    &self,
    path: &Path,
    reader: R,
  ) -> Result<()>;
  async fn stream_read_file(&self, path: &Path) -> Result<Box<dyn AsyncRead + Unpin + Send>>;
  async fn delete_file(&self, path: &Path) -> Result<()>;

  async fn write_file<D: AsRef<[u8]>>(&self, path: &Path, data: D) -> Result<()> {
    self.stream_write_file(path, data.as_ref()).await
  }

  async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
    let mut reader = self.stream_read_file(path).await?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    Ok(buf)
  }
}
