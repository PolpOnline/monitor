use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgInterval;
use time::{Duration, OffsetDateTime};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    users::AuthSession,
    web::{
        protected::list_systems::Visibility, utils::time_conversions::offset_to_primitive_date_time,
    },
};

#[derive(Debug, Deserialize, Clone, TS)]
#[ts(export)]
pub struct AddSystemRequest {
    name: String,
    /// Frequency in minutes
    frequency: i64,
    #[serde(with = "time::serde::iso8601")]
    #[ts(type = "string")]
    starts_at: OffsetDateTime,
    /// Time in minutes after which the user will get emailed
    down_after: i64,
    visibility: Visibility,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct AddSystemResponse {
    id: Uuid,
}

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

    let starts_at = offset_to_primitive_date_time(request.starts_at);

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

    Json(AddSystemResponse { id }).into_response()
}
