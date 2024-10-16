use axum::response::IntoResponse;
use http::StatusCode;

pub async fn healthcheck() -> impl IntoResponse {
    StatusCode::OK
}
