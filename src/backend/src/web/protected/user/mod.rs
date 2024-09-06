mod change_password;

use axum::{routing::patch, Router};

pub fn router() -> Router<()> {
    Router::new().route(
        "/user/change_password",
        patch(change_password::change_password),
    )
}
