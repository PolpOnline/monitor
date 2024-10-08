use axum::{response::IntoResponse, Json};
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use password_auth::generate_hash;
use serde::Deserialize;
use thiserror::Error;
use tokio::task;
use ts_rs::TS;

use crate::users::{AuthSession, Credentials};

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum ChangePasswordError {
    #[error("Failed to generate hash")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToGenerateHash,
    #[error("User not logged in")]
    #[status(StatusCode::UNAUTHORIZED)]
    UserNotLoggedIn,
    #[error("Old password is wrong")]
    #[status(StatusCode::UNAUTHORIZED)]
    OldPasswordIsWrong,
    #[error("Failed to authenticate with old password")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToAuthenticateWithOldPassword,
    #[error("Failed to update password")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToUpdatePassword,
}

pub async fn change_password(
    auth_session: AuthSession,
    Json(request): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    let current_user = match auth_session.user {
        Some(ref user) => user,
        None => return ChangePasswordError::UserNotLoggedIn.into_response(),
    };

    let credentials_to_authenticate = Credentials {
        email: current_user.email.clone(),
        password: request.old_password.clone(),
    };

    match auth_session.authenticate(credentials_to_authenticate).await {
        Ok(Some(_)) => {}
        Ok(None) => return ChangePasswordError::OldPasswordIsWrong.into_response(),
        Err(_) => return ChangePasswordError::FailedToAuthenticateWithOldPassword.into_response(),
    }

    let new_encrypted_password =
        match task::spawn_blocking(move || generate_hash(request.new_password.as_bytes())).await {
            Ok(p) => p,
            Err(_) => return ChangePasswordError::FailedToGenerateHash.into_response(),
        };

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        UPDATE "user" SET password = $1 WHERE id = $2
        "#,
        new_encrypted_password,
        current_user.id
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return ChangePasswordError::FailedToUpdatePassword.into_response(),
    }

    StatusCode::OK.into_response()
}
