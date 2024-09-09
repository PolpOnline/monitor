use axum::{
    routing::{delete, get, post},
    Router,
};

mod add_system;
mod delete_system;
mod list_systems;
mod user;

pub fn router() -> Router<()> {
    Router::new()
        .merge(user::router())
        .route("/add_system", post(add_system::add_system))
        .route("/delete_system", delete(delete_system::delete_system))
        .route("/list_systems", get(list_systems::list_systems))
}
