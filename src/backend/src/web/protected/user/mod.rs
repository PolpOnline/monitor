mod change_language;
mod change_password;
mod change_timezone;
mod get_current_settings;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![change_password::change_password])
        .routes(routes![change_timezone::change_timezone])
        .routes(routes![get_current_settings::get_current_settings])
        .routes(routes![change_language::change_language])
}
