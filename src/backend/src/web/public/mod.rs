mod ping_status;
mod public_systems;
mod user_info;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/ping_status/:id", post(ping_status::ping_status))
        .route("/user_info", get(user_info::user_info))
        .merge(public_systems::router())
}
