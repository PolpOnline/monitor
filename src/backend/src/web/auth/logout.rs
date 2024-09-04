use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

use crate::users::AuthSession;

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub status: String,
}

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
