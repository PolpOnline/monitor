use crate::users::{AuthSession, Credentials};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
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

mod post {
    use super::*;
    use axum::Json;
    use axum_login::tracing::debug;

    pub async fn login(
        mut auth_session: AuthSession,
        Json(req): Json<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(req.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                println!("Invalid credentials");

                let mut login_url = "/login".to_string();
                if let Some(next) = req.next {
                    login_url = format!("{}?next={}", login_url, next);
                };

                return Redirect::to(&login_url).into_response();
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
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
