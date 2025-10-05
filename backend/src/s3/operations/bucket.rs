use axum::{
  Router,
  extract::Path,
  routing::{delete, get, put},
};
use centaurus::{error::Result, req::xml::Xml};
use http::{HeaderMap, header::CONTENT_TYPE};
use serde::Deserialize;

use crate::s3::{
  auth::{Identity, S3Auth},
  interface::S3Interface,
};

pub fn router() -> Router {
  Router::new()
    .route("/{*bucket}", put(create_bucket))
    .route("/{*bucket}", delete(delete_bucket))
    .route("/", get(list_buckets))
}

/// TODO: Handling of additional header options
async fn create_bucket(
  interface: S3Interface,
  Path(bucket): Path<String>,
  S3Auth { body, identity, .. }: S3Auth<Option<Xml<CreateBucketConfiguration>>>,
) -> Result<HeaderMap> {
  dbg!(&body);
  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} creating bucket {bucket}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to create bucket");
    }
  }

  interface.create_bucket(&bucket).await?;

  let mut headers = HeaderMap::new();
  headers.insert("Location", format!("/{bucket}").parse()?);

  Ok(headers)
}

async fn delete_bucket(
  interface: S3Interface,
  Path(bucket): Path<String>,
  S3Auth { identity, .. }: S3Auth<()>,
) -> Result<()> {
  let bucket = bucket.trim_end_matches('/').to_string();

  match identity {
    Identity::AccessKey(key) => {
      tracing::info!("AccessKey {key} deleting bucket {bucket}");
    }
    Identity::Anonymous => {
      tracing::warn!("Anonymous access to delete bucket");
    }
  }

  interface.delete_bucket(&bucket).await?;

  Ok(())
}

async fn list_buckets(interface: S3Interface) -> Result<(HeaderMap, String)> {
  let buckets = interface.list_buckets().await?;

  // einfache XML-Escaping-Funktion
  fn xml_escape(s: &str) -> String {
    s.chars()
      .map(|c| match c {
        '&' => "&amp;".into(),
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '"' => "&quot;".into(),
        '\'' => "&apos;".into(),
        other => other.to_string(),
      })
      .collect()
  }

  let mut xml = String::new();
  xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
  xml.push_str(r#"<ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">"#);
  xml.push_str("<Buckets>");
  for name in buckets {
    xml.push_str("<Bucket>");
    // nur Name + CreationDate minimal (CreationDate leer, kann ggf. mit echtem Timestamp gefüllt werden)
    xml.push_str(&format!("<Name>{}</Name>", xml_escape(&name)));
    xml.push_str("<CreationDate></CreationDate>");
    xml.push_str("</Bucket>");
  }
  xml.push_str("</Buckets>");
  // einfacher Owner-Block; anpassen falls echte Owner-Infos vorhanden sind
  xml.push_str("<Owner><DisplayName>owner</DisplayName><ID>ownerid</ID></Owner>");
  xml.push_str("</ListAllMyBucketsResult>");

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());

  Ok((headers, xml))
}

/// TODO: Handling of additional configuration options
#[derive(Deserialize, Debug)]
struct CreateBucketConfiguration {}
