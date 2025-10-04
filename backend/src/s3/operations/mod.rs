use axum::Router;

mod bucket;


pub fn router() -> Router {
  bucket::router()
}
