use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    users::AuthSession,
    web::protected::list_systems::{SystemData, SystemRecord, Visibility, LIMIT_SYSTEM_REQUEST},
};

#[derive(Debug, Deserialize, Clone)]
pub struct GetPublicQuery {
    pub list_size: i64,
    pub page: i64,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct GetPublicResponse {
    pub system: SystemData,
}

pub async fn get_public(
    auth_session: AuthSession,
    Path(uuid): Path<Uuid>,
    Query(query): Query<GetPublicQuery>,
) -> impl IntoResponse {
    if query.list_size > LIMIT_SYSTEM_REQUEST {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let db_system = match sqlx::query_as!(
        SystemRecord,
        // language=PostgreSQL
        r#"
        SELECT id, name, user_id, frequency, starts_at, deleted, down_after, down_sent_email, visibility AS "visibility: Visibility"
        FROM system WHERE id = $1 AND visibility = 'public'
        "#,
        uuid
    )
    .fetch_one(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(sqlx::error::Error::RowNotFound) => {
            return StatusCode::NOT_FOUND.into_response()
        }
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let system_data = match SystemData::fetch_from_db(
        &auth_session.backend.db,
        query.list_size,
        query.page,
        db_system,
    )
    .await
    {
        Ok(r) => r,
        Err(s) => return s.into_response(),
    };

    Json(GetPublicResponse {
        system: system_data,
    })
    .into_response()
}
