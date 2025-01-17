use axum::{response::IntoResponse, Json};
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use password_auth::generate_hash;
use serde::Deserialize;
use thiserror::Error;
use tokio::task;
use utoipa::ToSchema;

use crate::users::{AuthSession, Credentials};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// The old password
    old_password: String,
    /// The new password
    new_password: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum ChangePasswordError {
    #[error("Failed to generate hash")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToGenerateHash,
    #[error("User is not logged in")]
    #[status(StatusCode::UNAUTHORIZED)]
    UserNotLoggedIn,
    #[error("Old password is wrong")]
    #[status(StatusCode::FORBIDDEN)]
    OldPasswordIsWrong,
    #[error("Failed to authenticate with old password")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToAuthenticateWithOldPassword,
    #[error("Failed to update password")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToUpdatePassword,
}

#[utoipa::path(
    patch,
    path = "/change_password",
    summary = "Change Password",
    request_body = ChangePasswordRequest,
    responses(
        (status = OK, description = "Password was changed successfully"),
        (status = UNAUTHORIZED, description = "User is not logged in", body = str, example = "User is not logged in"),
        (status = FORBIDDEN, description = "Old password is wrong", body = str, example = "Old password is wrong"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = str)
    ),
    security(
        ("session" = [])
    ),
    tag = "User"
)]
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
