mod ping_status;
mod public_systems;

use axum::{routing::post, Router};

pub fn router() -> Router<()> {
    Router::new()
        .route("/ping_status/:id", post(ping_status::ping_status))
        .merge(public_systems::router())
}
