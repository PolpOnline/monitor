use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
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
