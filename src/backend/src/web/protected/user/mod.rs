mod change_password;
mod change_timezone;
mod get_current_settings;

use axum::{
    routing::{get, patch},
    Router,
};

pub fn router() -> Router<()> {
    Router::new()
        .route(
            "/user/change_password",
            patch(change_password::change_password),
        )
        .route(
            "/user/change_timezone",
            patch(change_timezone::change_timezone),
        )
        .route(
            "/user/get_current_settings",
            get(get_current_settings::get_current_settings),
        )
}
