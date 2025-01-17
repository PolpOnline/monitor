use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;
use tracing::info;
use uuid::Uuid;

use crate::{app::openapi::DATA_TAG, users::AuthSession};

#[utoipa::path(
    post,
    path = "/ping_status/{id}",
    summary = "Ping as system",
    description = "Ping this endpoint to update the status of the system",
    responses(
        (status = OK, description = "Ping was successful"),
        (status = NOT_FOUND, description = "System not found"),
    ),
    tag = DATA_TAG
)]
pub async fn ping_status(Path(id): Path<Uuid>, auth_session: AuthSession) -> impl IntoResponse {
    info!("System {} just pinged!", id);

    // Check if the system exists and is not deleted
    if sqlx::query!(
        r#"
        SELECT id FROM system WHERE id = $1 AND deleted = false
        "#,
        id
    )
    .fetch_optional(&auth_session.backend.db)
    .await
    .is_err()
    {
        return StatusCode::NOT_FOUND.into_response();
    };

    // Insert the ping into the database and reset the down_sent_email flag
    sqlx::query!(
        r#"
        WITH updated AS (
            UPDATE system
            SET down_sent_email = false
            WHERE id = $1
        )
        INSERT INTO ping (system_id) VALUES ($1)
        "#,
        id
    )
    .execute(&auth_session.backend.db)
    .await
    .unwrap();

    StatusCode::OK.into_response()
}
