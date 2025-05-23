mod db;
pub mod openapi;
mod redis;
mod workers;

use std::str::FromStr;

use axum::{middleware, routing::get};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{Expiry, SessionManagerLayer},
};
use http::StatusCode;
use sqlx::PgPool;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    decompression::{DecompressionLayer, RequestDecompressionLayer},
    trace::TraceLayer,
};
use tower_sessions::cookie::Key;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool as FredPool};
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    app::{openapi::ApiDoc, redis::RedisLibPool},
    custom_login_required,
    middleware::{set_cache_control::set_cache_control, set_user_info::set_user_info},
    users::LoginBackend,
    web::{auth, protected, public},
    workers::email_worker::{SmtpClient, init_smtp_client},
};

pub struct App {
    db: PgPool,
    redis_lib: RedisLibPool,
    redis_fred: FredPool,
    smtp_client: SmtpClient,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let (db, redis_lib, redis_fred, smtp_client) = tokio::try_join!(
            Self::setup_db(),
            Self::setup_redis_lib(),
            Self::setup_redis_fred(),
            init_smtp_client()
        )?;

        Ok(Self {
            db,
            redis_lib,
            redis_fred,
            smtp_client,
        })
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        // Worker task.
        let worker_task_handle = {
            let processor =
                Self::init_workers(self.redis_lib.clone(), self.db.clone(), self.smtp_client)
                    .await?;

            tokio::task::spawn(Self::start_workers(processor))
        };

        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_layer = {
            let session_store = RedisStore::new(self.redis_fred.clone());

            // Get the cookie key from the environment.
            let key = &std::env::var("COOKIE_KEY")?;
            let key = parse_cookie_key(key);

            SessionManagerLayer::new(session_store)
                .with_name("monitor_id")
                .with_secure(true)
                .with_expiry(Expiry::OnInactivity(
                    tower_sessions::cookie::time::Duration::days(7),
                ))
                .with_signed(key)
        };

        // Auth service.
        //
        // This combines the session layer with our backendOld to establish the auth
        // service which will provide the auth session as a request extension.
        let auth_layer = {
            let backend = LoginBackend::new(self.db);
            AuthManagerLayerBuilder::new(backend, session_layer).build()
        };

        let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(protected::router())
            .route_layer(custom_login_required!(
                LoginBackend,
                (StatusCode::UNAUTHORIZED, "You are not logged in.")
            ))
            .merge(auth::router())
            .merge(public::router())
            .layer(middleware::from_fn(set_user_info))
            .layer(auth_layer)
            .layer(middleware::from_fn(set_cache_control))
            .split_for_parts();

        let router = {
            let api_json = sonic_rs::to_value(&api.clone()).expect("Failed to convert api to JSON");

            router
                .route(
                    "/openapi.json",
                    get(move || async { axum_serde::Sonic(api_json) }),
                )
                .merge(Scalar::with_url("/scalar", api))
        };

        let router = router
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(RequestDecompressionLayer::new())
            .layer(DecompressionLayer::new());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Axum: Listening on {}", listener.local_addr()?);

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        let handles = vec![worker_task_handle];

        // Abort each worker
        for handle in handles.iter() {
            handle.abort_handle().abort();
        }

        // Wait for each worker to finish.
        futures::future::join_all(handles).await;

        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutting down...");
}

fn parse_cookie_key(cookie_key: &str) -> Key {
    let key: Vec<u8> = cookie_key[1..cookie_key.len() - 1]
        .split(", ")
        .filter_map(|byte| u8::from_str(byte.trim()).ok())
        .collect();

    Key::from(&key)
}
