mod get_public;

use axum::{routing::get, Router};

pub fn router() -> Router<()> {
    Router::new().route("/get_public/:id", get(get_public::get_public))
}
