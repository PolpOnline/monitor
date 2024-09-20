use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::{users::AuthSession, web::protected::list_systems::Visibility};

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ChangeVisibilityRequest {
    id: Uuid,
    visibility: Visibility,
}

pub async fn change_visibility(
    auth_session: AuthSession,
    Json(request): Json<ChangeVisibilityRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        UPDATE system SET visibility = ($1) WHERE id = $2  
        "#,
        request.visibility as Visibility,
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
