use axum::response::IntoResponse;
use http::StatusCode;

use crate::{app::openapi::AUTH_TAG, users::AuthSession};

#[utoipa::path(
    get,
    path = "/logout",
    summary = "Logout",
    description = "Logout the current user",
    responses(
        (status = OK, description = "User was logged out"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = AUTH_TAG
)]
pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    if auth_session.logout().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::OK.into_response()
}
