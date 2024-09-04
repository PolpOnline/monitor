use axum::{routing::get, Router};

mod hello;

pub fn router() -> Router<()> {
    Router::new().route("/", get(hello::hello))
}
