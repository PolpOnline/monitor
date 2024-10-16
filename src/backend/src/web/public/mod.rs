mod healthcheck;
mod ping_status;
mod public_systems;
mod sys_info;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/ping_status/:id", post(ping_status::ping_status))
        .route("/sys_info", get(sys_info::sys_info))
        .route("/healthcheck", get(healthcheck::healthcheck))
        .merge(public_systems::router())
}
