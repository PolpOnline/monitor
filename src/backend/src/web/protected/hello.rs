use crate::users::AuthSession;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Protected {
    pub status: String,
}

pub async fn hello(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => {
            let res = Protected {
                status: format!("Hello, {}!", user.username),
            };
            Json(res).into_response()
        }

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
