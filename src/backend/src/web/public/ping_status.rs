use axum::{extract::Path, response::IntoResponse};
use http::StatusCode;
use tracing::info;
use uuid::Uuid;

pub async fn ping_status(Path(id): Path<Uuid>) -> impl IntoResponse {
    info!("Ping status for id: {}", id);

    StatusCode::OK.into_response()
}
