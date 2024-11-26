mod login;
mod logout;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![login::login])
        .routes(routes![logout::logout])
}
