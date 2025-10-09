use std::time::Duration;

use crate::common::run;

mod common;

#[tokio::test]
async fn health_check_works() {
  let ports = run().await;
  dbg!(ports.backend_port);

  let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .connect_timeout(Duration::from_secs(10))
    .build()
    .unwrap();

  let response = client
    .get(format!(
      "http://localhost:{}/api/health",
      ports.backend_port
    ))
    .send()
    .await
    .unwrap();
  assert!(response.status().is_success());
}
