use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::AUTH_TAG, users::AuthSession};

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/logout",
    responses(
        (status = OK, description = "User was logged out"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    tag = AUTH_TAG
)]
pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => {
            let res = LogoutResponse {
                status: "success".to_string(),
            };
            Json(res).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
