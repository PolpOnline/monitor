use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::users::AuthSession;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct EditSystemNameRequest {
    name: String,
    id: Uuid,
}

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
