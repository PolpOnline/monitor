use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app::openapi::SYSTEM_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteSystemRequest {
    /// The ID of the system to delete
    id: Uuid,
}

#[utoipa::path(
    delete,
    path = "/delete_system",
    summary = "Delete System",
    request_body = DeleteSystemRequest,
    responses(
        (status = OK, description = "System was deleted successfully"),
        (status = UNAUTHORIZED, description = "User is not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = SYSTEM_TAG
)]
pub async fn delete_system(
    auth_session: AuthSession,
    Sonic(request): Sonic<DeleteSystemRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    if sqlx::query!(
        r#"
        UPDATE system SET deleted = true WHERE id = $1  
        "#,
        request.id,
    )
    .execute(&auth_session.backend.db)
    .await
    .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    StatusCode::OK.into_response()
}
