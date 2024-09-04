mod ping_status;

use axum::{routing::post, Router};

pub fn router() -> Router<()> {
    Router::new().route("/ping_status/:id", post(ping_status::ping_status))
}
