use axum::response::IntoResponse;
use http::StatusCode;

use crate::app::MONITORING_TAG;

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = OK, description = "Healthcheck was successful")
    ),
    tag = MONITORING_TAG
)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
