use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use tracing::{debug, info};

use crate::users::{AuthSession, Credentials};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: String,
}

pub async fn login(
    mut auth_session: AuthSession,
    Json(req): Json<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(req.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            debug!("Failed to authenticate as {}", req.email);
            return (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse {
                    status: "failed".to_string(),
                }),
            )
                .into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    info!("Successfully logged in as {}", user.email);

    Json(LoginResponse {
        status: "success".to_string(),
    })
    .into_response()
}
