use std::{fmt::Debug, path::PathBuf};

use ichwilldich_lib::{error::Result, req::xml::Xml};
use serde::Deserialize;
use tokio::{
  fs::{File, OpenOptions},
  io::AsyncWriteExt,
};
use uuid::Uuid;

pub trait Body: Sized {
  type Writer: BodyWriter + Send;

  fn from_writer(writer: Self::Writer) -> Result<Self>;
}

/// A temporary file that will be deleted when dropped
/// This should be used for file uploads because they can be large
#[allow(unused)]
pub struct TmpFile(pub PathBuf);
#[derive(Debug)]
#[allow(unused)]
pub struct FileWriter(File, PathBuf);

impl Drop for TmpFile {
  fn drop(&mut self) {
    if self.0.exists() {
      let _ = std::fs::remove_file(&self.0);
    }
  }
}

impl Body for () {
  type Writer = ();

  fn from_writer(_: Self::Writer) -> Result<Self> {
    Ok(())
  }
}

impl Body for TmpFile {
  type Writer = FileWriter;

  fn from_writer(writer: Self::Writer) -> Result<Self> {
    Ok(TmpFile(writer.1))
  }
}

impl Body for Vec<u8> {
  type Writer = Vec<u8>;

  fn from_writer(writer: Self::Writer) -> Result<Self> {
    Ok(writer)
  }
}

impl<T: Body> Body for Option<T> {
  type Writer = T::Writer;

  fn from_writer(writer: Self::Writer) -> Result<Self> {
    Ok(T::from_writer(writer).ok())
  }
}

impl<T> Body for Xml<T>
where
  T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
  type Writer = Vec<u8>;

  fn from_writer(writer: Self::Writer) -> Result<Self> {
    Ok(Xml::from_slice(&writer)?)
  }
}

#[async_trait::async_trait]
pub trait BodyWriter: Sized + Debug {
  async fn new() -> Result<Self>;
  async fn write(&mut self, buf: &[u8]) -> Result<()>;
}

#[async_trait::async_trait]
impl BodyWriter for () {
  async fn new() -> Result<Self> {
    Ok(())
  }

  async fn write(&mut self, _: &[u8]) -> Result<()> {
    Ok(())
  }
}

#[async_trait::async_trait]
impl BodyWriter for FileWriter {
  async fn new() -> Result<Self> {
    let path = std::env::temp_dir().join(format!("ichwilldich-{}", Uuid::new_v4()));
    let file = OpenOptions::new()
      .create_new(true)
      .read(true)
      .append(true)
      .open(&path)
      .await?;
    Ok(FileWriter(file, path))
  }

  async fn write(&mut self, buf: &[u8]) -> Result<()> {
    self.0.write_all(buf).await?;

    Ok(())
  }
}

#[async_trait::async_trait]
impl BodyWriter for Vec<u8> {
  async fn new() -> Result<Self> {
    Ok(Vec::new())
  }

  async fn write(&mut self, buf: &[u8]) -> Result<()> {
    self.extend_from_slice(buf);
    Ok(())
  }
}
