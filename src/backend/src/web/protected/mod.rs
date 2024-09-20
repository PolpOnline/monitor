use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub mod add_system;
pub mod change_visibility;
pub mod delete_system;
pub mod edit_system_name;
pub mod list_systems;
pub mod user;

pub fn router() -> Router<()> {
    Router::new()
        .merge(user::router())
        .route("/add_system", post(add_system::add_system))
        .route("/delete_system", delete(delete_system::delete_system))
        .route("/list_systems/:list_size", get(list_systems::list_systems))
        .route(
            "/edit_system_name",
            patch(edit_system_name::edit_system_name),
        )
        .route(
            "/change_visibility",
            patch(change_visibility::change_visibility),
        )
}
