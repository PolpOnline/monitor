use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::users::AuthSession;

#[derive(Serialize, Debug, ToSchema)]
pub struct GetCurrentSettingsResponse {
    /// The current timezone of the user, as defined by the IANA Time Zone
    /// Database
    pub timezone: String,
    /// The current language of the user, as defined by the IETF language tag
    pub language: String,
}

#[utoipa::path(
    get,
    path = "/get_current_settings",
    summary = "Retrieve Settings",
    responses(
        (status = OK, description = "Current settings were retrieved successfully", body = GetCurrentSettingsResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = "User"
)]
pub async fn get_current_settings(auth_session: AuthSession) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let current_settings = match sqlx::query!(
        r#"
        SELECT timezone, language FROM "user" WHERE id = $1
        "#,
        current_user.id
    )
    .fetch_one(&auth_session.backend.db)
    .await
    {
        Ok(settings) => settings,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let response = GetCurrentSettingsResponse {
        timezone: current_settings.timezone,
        language: current_settings.language,
    };

    Sonic(response).into_response()
}
