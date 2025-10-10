use backend::App;
#[cfg(debug_assertions)]
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let app = App::new().await;
  app.run().await;
}
