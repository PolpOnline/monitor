use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;
use tracing::info;
use uuid::Uuid;

use crate::users::AuthSession;

pub async fn ping_status(Path(id): Path<Uuid>, auth_session: AuthSession) -> impl IntoResponse {
    info!("System {} just pinged!", id);

    sqlx::query!(
        // language=PostgreSQL
        r#"
        INSERT INTO ping (system_id) VALUES ($1)
        "#,
        id
    )
    .execute(&auth_session.backend.db)
    .await
    .unwrap();

    StatusCode::OK.into_response()
}
