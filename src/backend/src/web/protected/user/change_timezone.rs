use std::str::FromStr;

use axum::{response::IntoResponse, Json};
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ChangeTimezoneRequest {
    timezone: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum ChangePasswordError {
    #[error("User not logged in")]
    #[status(StatusCode::UNAUTHORIZED)]
    UserNotLoggedIn,
    #[error("Timezone not valid")]
    #[status(StatusCode::BAD_REQUEST)]
    TimeZoneNotValid,
    #[error("Failed to update timezone")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToUpdateTimezone,
}

pub async fn change_timezone(
    auth_session: AuthSession,
    Json(request): Json<ChangeTimezoneRequest>,
) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return ChangePasswordError::UserNotLoggedIn.into_response(),
    };

    let tz = match chrono_tz::Tz::from_str(&request.timezone) {
        Ok(tz) => tz,
        Err(_) => return ChangePasswordError::TimeZoneNotValid.into_response(),
    };

    match sqlx::query!(
        // language=PostgreSQL
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
        Err(_) => return ChangePasswordError::FailedToUpdateTimezone.into_response(),
    }

    StatusCode::OK.into_response()
}
