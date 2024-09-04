use axum::{
    routing::{get, post},
    Router,
};

mod add_system;
mod hello;

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(hello::hello))
        .route("/add_system", post(add_system::add_system))
}
