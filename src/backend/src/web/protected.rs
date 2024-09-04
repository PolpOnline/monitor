use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Serialize;

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/", get(get::protected))
}

#[derive(Debug, Serialize, Clone)]
pub struct Protected {
    pub status: String,
}

mod get {
    use super::*;
    use axum::Json;

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
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
}
