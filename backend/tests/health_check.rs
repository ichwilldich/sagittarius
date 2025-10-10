use crate::common::run;

mod common;

#[tokio::test]
async fn health_check_works() {
  let ports = run().await;

  let start = std::time::Instant::now();
  let response = common::reqwest_client()
    .get(format!(
      "http://localhost:{}/api/health",
      ports.backend_port
    ))
    .send()
    .await
    .unwrap();
  println!("Health check in {:?}", start.elapsed());
  assert!(response.status().is_success());
}
