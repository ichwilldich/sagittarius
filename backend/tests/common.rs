use std::{env::set_var, net::TcpListener};

use tokio::spawn;

fn find_port(used: Option<u16>) -> u16 {
  (8000..16000)
    .find(|port| used != Some(*port) && TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok())
    .expect("No ports free")
}

pub fn prepare_env() -> Ports {
  let backend_port = find_port(None);
  let s3_port = find_port(Some(backend_port));
  let storage_path = format!("/tmp/sagittarius-test-{}", backend_port);

  unsafe {
    set_var("PORT", backend_port.to_string());
    set_var("S3_PORT", s3_port.to_string());
    set_var("STORAGE_PATH", &storage_path);
  }

  Ports {
    backend_port,
    s3_port,
  }
}

pub struct Ports {
  pub backend_port: u16,
  #[allow(unused)]
  pub s3_port: u16,
}

pub async fn launch_app() {
  spawn(backend::app());
}

pub async fn run() -> Ports {
  let ports = prepare_env();
  launch_app().await;
  ports
}
