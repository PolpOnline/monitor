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
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
