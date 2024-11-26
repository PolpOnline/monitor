use axum::response::IntoResponse;
use http::StatusCode;

use crate::app::MONITORING_TAG;

#[utoipa::path(
    get,
    path = "/healthcheck",
    description = "Check if the server is running",
    responses(
        (status = OK, description = "Healthcheck was successful")
    ),
    tag = MONITORING_TAG
)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
