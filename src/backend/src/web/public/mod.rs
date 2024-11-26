mod healthcheck;
mod ping_status;
mod public_systems;
mod sys_info;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![ping_status::ping_status])
        .routes(routes![sys_info::sys_info])
        .routes(routes![healthcheck::healthcheck])
        .merge(public_systems::router())
}
