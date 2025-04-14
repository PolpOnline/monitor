use std::str::FromStr;

use axum::response::IntoResponse;
use axum_serde::Sonic;
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::users::AuthSession;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangeTimezoneRequest {
    /// The new timezone, as defined by the IANA Time Zone Database
    timezone: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum ChangeTimezoneError {
    #[error("User is not logged in")]
    #[status(StatusCode::UNAUTHORIZED)]
    UserNotLoggedIn,
    #[error("Timezone not valid")]
    #[status(StatusCode::BAD_REQUEST)]
    TimeZoneNotValid,
    #[error("Failed to update timezone")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToUpdateTimezone,
}

#[utoipa::path(
    patch,
    path = "/change_timezone",
    summary = "Change Timezone",
    request_body = ChangeTimezoneRequest,
    responses(
        (status = OK, description = "Timezone was changed successfully"),
        (status = UNAUTHORIZED, description = "User is not logged in", body = str, example = "User is not logged in"),
        (status = BAD_REQUEST, description = "Timezone is not valid", body = str, example = "Timezone not valid"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = str, example = "Failed to update timezone")
    ),
    security(
        ("session" = [])
    ),
    tag = "User"
)]
pub async fn change_timezone(
    auth_session: AuthSession,
    Sonic(request): Sonic<ChangeTimezoneRequest>,
) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return ChangeTimezoneError::UserNotLoggedIn.into_response(),
    };

    let tz = match chrono_tz::Tz::from_str(&request.timezone) {
        Ok(tz) => tz,
        Err(_) => return ChangeTimezoneError::TimeZoneNotValid.into_response(),
    };

    match sqlx::query!(
        r#"
        UPDATE "user" SET timezone = $1 WHERE id = $2
        "#,
        tz.to_string(),
        current_user.id
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return ChangeTimezoneError::FailedToUpdateTimezone.into_response(),
    }

    StatusCode::OK.into_response()
}
