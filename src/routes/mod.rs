use axum::Router;
use axum::routing::get;
use crate::handlers::hello_world;

pub fn create_router() -> Router<> {
    Router::new().route("/", get(hello_world))
}