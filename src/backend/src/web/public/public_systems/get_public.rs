use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    users::AuthSession,
    web::protected::list_systems::{SystemData, SystemRecord, Visibility},
};

#[derive(Debug, Deserialize, Clone)]
pub struct GetPublicRequest {
    pub list_size: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct GetPublicResponse {
    pub system: SystemData,
}

pub async fn get_public(
    auth_session: AuthSession,
    Path(uuid): Path<Uuid>,
    Query(req): Query<GetPublicRequest>,
) -> impl IntoResponse {
    let db_system = match sqlx::query_as!(
        SystemRecord,
        // language=PostgreSQL
        r#"
        SELECT id, name, user_id, frequency, starts_at, deleted, down_after, down_sent_email, visibility AS "visibility: Visibility"
        FROM system WHERE id = $1
        "#,
        uuid
    )
    .fetch_one(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let system_data =
        match SystemData::fetch_from_db(&auth_session.backend.db, req.list_size, db_system).await {
            Ok(r) => r,
            Err(s) => return s.into_response(),
        };

    Json(GetPublicResponse {
        system: system_data,
    })
    .into_response()
}
