use axum::{
    routing::{delete, get, post},
    Router,
};

mod add_system;
mod list_systems;
mod remove_system;

pub fn router() -> Router<()> {
    Router::new()
        .route("/add_system", post(add_system::add_system))
        .route("/remove_system", delete(remove_system::remove_system))
        .route("/list_systems", get(list_systems::list_systems))
}
