use crate::users::{AuthSession, Credentials};
use axum::Json;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_login::tracing::debug;
use serde::Serialize;

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(post::login))
        .route("/logout", get(get::logout))
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub status: String,
}

mod post {
    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        Json(req): Json<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(req.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                debug!("Failed to authenticate as {}", req.username);
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

        debug!("Successfully logged in as {}", user.username);

        Json(LoginResponse {
            status: "success".to_string(),
        })
        .into_response()
    }
}

mod get {
    use super::*;

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
}
