use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;

use crate::users::AuthSession;

#[derive(Debug, Serialize, Clone)]
pub struct Protected {
    pub status: String,
}

pub async fn hello(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => {
            let res = Protected {
                status: format!("Hello, {}!", user.email),
            };
            Json(res).into_response()
        }

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
