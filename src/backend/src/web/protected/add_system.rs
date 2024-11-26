use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Duration, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgInterval;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app::SYSTEM_TAG, users::AuthSession, web::protected::list_systems::Visibility};

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct AddSystemRequest {
    /// The name of the system
    name: String,
    /// The frequency in minutes of the pings
    frequency: i64,
    /// The time at which the system starts pinging
    starts_at: DateTime<Utc>,
    /// Time in minutes after which the user will get emailed
    down_after: i64,
    /// The visibility of the system
    visibility: Visibility,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct AddSystemResponse {
    /// The ID of the system that was created
    id: Uuid,
}

#[utoipa::path(
    post,
    path = "/add_system",
    description = "Add a new system",
    request_body = AddSystemRequest,
    responses(
        (status = CREATED, description = "System was created successfully", body = AddSystemResponse),
        (status = BAD_REQUEST, description = "Bad request"),
        (status = UNAUTHORIZED, description = "User is not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = SYSTEM_TAG
)]
pub async fn add_system(
    auth_session: AuthSession,
    Json(request): Json<AddSystemRequest>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let frequency: PgInterval = match Duration::minutes(request.frequency).try_into() {
        Ok(interval) => interval,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let starts_at = request.starts_at.naive_utc();

    let id = Uuid::new_v4();

    let down_after: PgInterval = match Duration::minutes(request.down_after).try_into() {
        Ok(interval) => interval,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        INSERT INTO system (id, name, user_id, frequency, starts_at, down_after, visibility)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        request.name,
        user.id,
        frequency,
        starts_at,
        down_after,
        request.visibility as Visibility
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    (StatusCode::CREATED, Json(AddSystemResponse { id })).into_response()
}
