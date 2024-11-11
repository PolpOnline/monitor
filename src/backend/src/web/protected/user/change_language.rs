use axum::{response::IntoResponse, Json};
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use rust_i18n::available_locales;
use serde::Deserialize;
use thiserror::Error;
use ts_rs::TS;

use crate::users::AuthSession;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ChangeLanguageRequest {
    language: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum ChangeLanguageError {
    #[error("User not logged in")]
    #[status(StatusCode::UNAUTHORIZED)]
    UserNotLoggedIn,
    #[error("Language isn't valid")]
    #[status(StatusCode::BAD_REQUEST)]
    LanguageNotValid,
    #[error("Failed to update language")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToUpdateLanguage,
}

pub async fn change_language(
    auth_session: AuthSession,
    Json(request): Json<ChangeLanguageRequest>,
) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return ChangeLanguageError::UserNotLoggedIn.into_response(),
    };

    let available_languages = available_locales!();

    if !available_languages.contains(&&*request.language) {
        return ChangeLanguageError::LanguageNotValid.into_response();
    }

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        UPDATE "user" SET language = $1 WHERE id = $2
        "#,
        request.language,
        current_user.id
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return ChangeLanguageError::FailedToUpdateLanguage.into_response(),
    }

    StatusCode::OK.into_response()
}
