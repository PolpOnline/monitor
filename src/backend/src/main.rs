//! Run with
//!
//! ```not_rust
//! cargo run -p example-sqlite
//! ```

use color_eyre::Result;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use rust_i18n::i18n;
use rustls::crypto::aws_lc_rs;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::App;

pub mod app;
pub mod middleware;
pub mod users;
pub mod web;
pub mod workers;

pub static PRODUCTION: Lazy<bool> = Lazy::new(|| std::env::var("PRODUCTION").is_ok());
pub static SITE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:5173".into()));

i18n!("./i18n/", fallback = ["en", "it"], minify_key = true);

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

    aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install AWS LC provider");

    if *PRODUCTION {
        info!("System: Production mode");
    } else {
        info!("System: Development mode");
    }

    App::new().await?.serve().await
}
