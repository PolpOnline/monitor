mod login;
mod logout;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(login::login))
        .route("/logout", get(logout::logout))
}
