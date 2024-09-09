use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::users::AuthSession;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct DeleteSystemRequest {
    id: Uuid,
}

pub async fn delete_system(
    auth_session: AuthSession,
    Json(request): Json<DeleteSystemRequest>,
) -> impl IntoResponse {
    if auth_session.user.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match sqlx::query!(
        // language=PostgreSQL
        r#"
        DELETE FROM system WHERE id = $1
        "#,
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
