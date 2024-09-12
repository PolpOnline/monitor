//! Run with
//!
//! ```not_rust
//! cargo run -p example-sqlite
//! ```

use color_eyre::Result;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::App;

mod users;
mod web;
mod workers;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=info,tower_sessions=info,sqlx=warn,tower_http=debug,monitor=debug"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    dotenv().unwrap_or_default();

    App::new().await?.serve().await
}
