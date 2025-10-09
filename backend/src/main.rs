#[cfg(debug_assertions)]
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  backend::app().await;
}
