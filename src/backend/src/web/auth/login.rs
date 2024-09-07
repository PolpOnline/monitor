use axum::{response::IntoResponse, Json};
use axum_thiserror::ErrorStatus;
use http::StatusCode;
use password_auth::generate_hash;
use serde::Serialize;
use thiserror::Error;
use tokio::task;
use tracing::{debug, info};
use ts_rs::TS;

use crate::users::{AuthSession, Credentials, User};

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct LoginResponse {
    pub status: String,
}

#[derive(Error, Debug, ErrorStatus)]
pub enum AuthError {
    #[error("Failed to generate hash")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToGenerateHash,
    #[error("Failed to insert user")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToInsertNewUser(#[from] sqlx::Error),
    #[error("User doesn't exist after signup")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    UserNotExistingAfterSignUp,
    #[error("Failed to re-authenticate after signup")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    FailedToReAuthenticateAfterSignUp,
    #[error("Wrong password")]
    #[status(StatusCode::UNAUTHORIZED)]
    WrongPassword,
}

pub async fn login(
    mut auth_session: AuthSession,
    Json(req): Json<Credentials>,
) -> impl IntoResponse {
    let mut status_code = StatusCode::OK;

    let user = match auth_session.authenticate(req.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let user_exists = get_user_existence(req.email.clone(), auth_session.clone()).await;

            if user_exists {
                debug!("Password is wrong for user {}", req.email);
                return AuthError::WrongPassword.into_response();
            } else {
                debug!(
                    "User does not exist, creating a new user with email {}",
                    req.email
                );
                match sign_up(req.clone(), auth_session.clone()).await {
                    Ok(user) => {
                        status_code = StatusCode::CREATED;
                        user
                    }
                    Err(e) => return e.into_response(),
                }
            }
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    info!("Successfully logged in as {}", user.email);

    match status_code {
        StatusCode::CREATED => (
            StatusCode::CREATED,
            Json(LoginResponse {
                status: "User was created".to_string(),
            }),
        )
            .into_response(),
        StatusCode::OK => Json(LoginResponse {
            status: "User was logged in".to_string(),
        })
        .into_response(),
        _ => unreachable!(),
    }
}

pub async fn re_auth_after_sign_up(
    auth_session: AuthSession,
    credentials: Credentials,
) -> Result<(), AuthError> {
    match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(_)) => Ok(()),
        Ok(None) => {
            debug!("User does not exist after sign up");
            Err(AuthError::UserNotExistingAfterSignUp)
        }
        Err(_) => Err(AuthError::FailedToReAuthenticateAfterSignUp),
    }
}

pub async fn get_user_existence(email: String, auth_session: AuthSession) -> bool {
    let user = sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT email
        FROM "user"
        WHERE email = $1;
        "#,
        email
    )
    .fetch_optional(&auth_session.backend.db)
    .await
    .expect("Failed to check if user exists");

    user.is_some()
}

pub async fn sign_up(
    credentials: Credentials,
    auth_session: AuthSession,
) -> Result<User, AuthError> {
    let password = credentials.password.clone();

    let encrypted_password = task::spawn_blocking(move || generate_hash(password.as_bytes()))
        .await
        .map_err(|_| AuthError::FailedToGenerateHash)?;

    let user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
        INSERT INTO "user" (email, password)
        VALUES ($1, $2)
        RETURNING id, email, password;
        "#,
        credentials.email,
        encrypted_password
    )
    .fetch_one(&auth_session.backend.db)
    .await?;

    re_auth_after_sign_up(auth_session, credentials).await?;

    Ok(user)
}
