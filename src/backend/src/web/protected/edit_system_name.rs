use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app::openapi::SYSTEM_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct EditSystemNameRequest {
    /// The ID of the system
    id: Uuid,
    /// The new name of the system
    name: String,
}

#[utoipa::path(
    patch,
    path = "/edit_system_name",
    request_body = EditSystemNameRequest,
    summary = "Edit Name",
    responses(
        (status = OK, description = "System name was edited successfully"),
        (status = UNAUTHORIZED, description = "User is not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = SYSTEM_TAG
)]
pub async fn edit_system_name(
    auth_session: AuthSession,
    Sonic(request): Sonic<EditSystemNameRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    if sqlx::query!(
        r#"
        UPDATE system SET name = ($1) WHERE id = $2  
        "#,
        request.name,
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
