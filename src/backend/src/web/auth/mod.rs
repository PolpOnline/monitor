mod login;
mod logout;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Serialize;

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(login::login))
        .route("/logout", get(logout::logout))
}
