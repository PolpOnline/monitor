use utoipa_axum::{router::OpenApiRouter, routes};

pub mod add_system;
pub mod change_visibility;
pub mod delete_system;
pub mod edit_system_name;
pub mod list_systems;
pub mod user;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/user", user::router())
        .routes(routes![add_system::add_system])
        .routes(routes![delete_system::delete_system])
        .routes(routes![list_systems::list_systems])
        .routes(routes![edit_system_name::edit_system_name])
        .routes(routes![change_visibility::change_visibility])
}
