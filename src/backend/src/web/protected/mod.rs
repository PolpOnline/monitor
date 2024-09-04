use axum::{
    routing::{delete, get, post},
    Router,
};

mod add_system;
mod hello;
mod remove_system;

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(hello::hello))
        .route("/add_system", post(add_system::add_system))
        .route("/remove_system", delete(remove_system::remove_system))
}
