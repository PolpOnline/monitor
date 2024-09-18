mod login_status;
mod ping_status;
mod public_systems;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/ping_status/:id", post(ping_status::ping_status))
        .route("/login_status", get(login_status::login_status))
        .merge(public_systems::router())
}
