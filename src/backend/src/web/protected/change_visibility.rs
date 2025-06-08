use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app::openapi::SYSTEM_TAG, users::AuthSession, web::protected::list_systems::Visibility,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangeVisibilityRequest {
    /// The ID of the system
    id: Uuid,
    /// The new visibility of the system
    visibility: Visibility,
}

#[utoipa::path(
    patch,
    path = "/change_visibility",
    summary = "Change Visibility",
    description = "Change the visibility of a system",
    request_body = ChangeVisibilityRequest,
    responses(
        (status = OK, description = "Visibility was changed successfully"),
        (status = UNAUTHORIZED, description = "User is not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = SYSTEM_TAG
)]
pub async fn change_visibility(
    auth_session: AuthSession,
    Sonic(request): Sonic<ChangeVisibilityRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    if sqlx::query!(
        r#"
        UPDATE system SET visibility = ($1) WHERE id = $2  
        "#,
        request.visibility as Visibility,
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
