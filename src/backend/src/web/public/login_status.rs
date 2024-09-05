use axum::{response::IntoResponse, Json};
use serde::Serialize;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub enum LoginStatus {
    #[serde(rename = "logged_in")]
    LoggedIn,
    #[serde(rename = "logged_out")]
    LoggedOut,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct LoginStatusResponse {
    pub status: LoginStatus,
}

pub async fn login_status(auth_session: AuthSession) -> impl IntoResponse {
    let status = match auth_session.user {
        Some(_) => LoginStatus::LoggedIn,
        None => LoginStatus::LoggedOut,
    };

    Json(LoginStatusResponse { status }).into_response()
}
