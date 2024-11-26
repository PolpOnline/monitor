use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app::SYSTEM_TAG, users::AuthSession};

#[derive(Debug, Deserialize, ToSchema)]
pub struct EditSystemNameRequest {
    name: String,
    id: Uuid,
}

#[utoipa::path(
    patch,
    path = "/edit_system_name",
    request_body = EditSystemNameRequest,
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
    Json(request): Json<EditSystemNameRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        UPDATE system SET name = ($1) WHERE id = $2  
        "#,
        request.name,
        request.id,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    StatusCode::OK.into_response()
}
