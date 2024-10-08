use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Serialize, Debug, TS)]
#[ts(export)]
pub struct GetCurrentSettingsResponse {
    pub timezone: String,
}

pub async fn get_current_settings(auth_session: AuthSession) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let timezone = match sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT timezone FROM "user" WHERE id = $1
        "#,
        current_user.id
    )
    .fetch_one(&auth_session.backend.db)
    .await
    {
        Ok(tz) => tz,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let timezone = timezone.timezone;

    let response = GetCurrentSettingsResponse { timezone };

    Json(response).into_response()
}
