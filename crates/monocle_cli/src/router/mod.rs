use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/ping", get(|| async { "" }))
}
