use crate::router::routes;
use axum::Router;

pub fn create_test_app() -> Router {
    routes()
}
