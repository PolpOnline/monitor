use axum::{response::IntoResponse, Json};
use serde::Serialize;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct UserInfoResponse {
    #[ts(type = "string | undefined")]
    pub email: Option<String>,
}

pub async fn user_info(auth_session: AuthSession) -> impl IntoResponse {
    let res = match auth_session.user {
        Some(user) => UserInfoResponse {
            email: Some(user.email.clone()),
        },
        None => UserInfoResponse { email: None },
    };

    Json(res).into_response()
}
