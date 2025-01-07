use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    app::openapi::PUBLIC_SYSTEM_TAG,
    users::AuthSession,
    web::protected::list_systems::{SystemData, SystemRecord, Visibility, LIMIT_SYSTEM_REQUEST},
};

#[derive(Debug, Deserialize, Clone, IntoParams)]
pub struct GetPublicQuery {
    /// The maximum number of instants to return
    pub list_size: i64,
    /// The page number to return
    pub page: i64,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct GetPublicResponse {
    /// The requested system's data
    pub system: SystemData,
}

#[utoipa::path(
    get,
    path = "/get_public/{id}",
    summary = "Retrieve Info",
    description = "Retrieve info about a public system",
    params(GetPublicQuery),
    responses(
        (status = OK, description = "Public system was retrieved successfully", body = GetPublicResponse),
        (status = BAD_REQUEST, description = "List size is too large"),
        (status = NOT_FOUND, description = "System not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    tag = PUBLIC_SYSTEM_TAG
)]
pub async fn get_public(
    auth_session: AuthSession,
    Path(uuid): Path<Uuid>,
    Query(query): Query<GetPublicQuery>,
) -> impl IntoResponse {
    if query.list_size > LIMIT_SYSTEM_REQUEST {
        return (StatusCode::BAD_REQUEST, "Limit of list_size exceeded").into_response();
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
