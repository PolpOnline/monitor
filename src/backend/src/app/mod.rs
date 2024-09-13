use std::str::FromStr;

use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use futures::TryFutureExt;
use http::StatusCode;
use sidekiq::{periodic, Processor, RedisConnectionManager};
use sqlx::{postgres::PgPoolOptions, PgPool};
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::{
    custom_login_required,
    users::LoginBackend,
    web::{auth, protected, public},
    workers::{
        email_worker::{init_smtp_client, SmtpClient},
        register_workers,
    },
};

type RedisPool = bb8::Pool<RedisConnectionManager>;

pub struct App {
    db: PgPool,
    redis: RedisPool,
    smtp_client: SmtpClient,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let db = Self::setup_db().await?;
        let redis = Self::setup_redis_lib().await?;
        let smtp_client = init_smtp_client()?;

        Ok(Self {
            db,
            redis,
            smtp_client,
        })
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = PostgresStore::new(self.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60))
                .map_err(color_eyre::Report::from),
        );

        let processor =
            Self::init_workers(self.redis.clone(), self.db.clone(), self.smtp_client).await?;

        let worker_task = tokio::task::spawn(Self::start_workers(processor));

        // Generate a cryptographic key to sign the session cookie.
        let key = &std::env::var("COOKIE_KEY")?;
        let key = parse_cookie_key(key);

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(Duration::days(7)))
            .with_signed(key);

        // Auth service.
        //
        // This combines the session layer with our backendOld to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = LoginBackend::new(self.db);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app = protected::router()
            .route_layer(custom_login_required!(
                LoginBackend,
                (StatusCode::UNAUTHORIZED, "You are not logged in.")
            ))
            .merge(auth::router())
            .merge(public::router())
            .layer(auth_layer)
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Axum: Listening on {}", listener.local_addr()?);

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(vec![
                deletion_task.abort_handle(),
                worker_task.abort_handle(),
            ]))
            .await?;

        futures::future::join_all(vec![deletion_task, worker_task]).await;

        Ok(())
    }

    async fn setup_db() -> color_eyre::Result<PgPool> {
        info!("SQLx: Connecting to the database...");

        let database_url = match std::env::var("DATABASE_PRIVATE_URL") {
            Ok(url) => {
                info!("SQLx: Using DATABASE_PRIVATE_URL");
                url
            }
            Err(_) => {
                info!("SQLx: Using DATABASE_URL");
                std::env::var("DATABASE_URL")?
            }
        };

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        info!("SQLx: Connected to the database");

        sqlx::migrate!().run(&pool).await?;

        info!("SQLx: Migrations run");

        Ok(pool)
    }

    async fn setup_redis_lib() -> color_eyre::Result<RedisPool> {
        info!("Redis: Connecting to Redis (to manage workers)...");

        let redis_url = std::env::var("REDIS_URL")?;
        let manager = RedisConnectionManager::new(redis_url)?;
        let redis = bb8::Pool::builder().build(manager).await?;

        info!("Redis: Connected to Redis (to manage workers)");

        Ok(redis)
    }

    async fn start_workers(p: Processor) -> color_eyre::Result<()> {
        info!("Sidekiq: Workers started");

        // Start the server
        p.run().await;

        Ok(())
    }

    async fn init_workers(
        redis: RedisPool,
        db: PgPool,
        smtp_client: SmtpClient,
    ) -> color_eyre::Result<Processor> {
        // Clear out all periodic jobs and their schedules
        periodic::destroy_all(redis.clone()).await?;

        // Sidekiq server
        let mut p = Processor::new(redis, vec!["down_emails".to_string()]);

        // Add known workers
        register_workers(&mut p, db, smtp_client).await?;

        Ok(p)
    }
}

async fn shutdown_signal(abort_handles: Vec<AbortHandle>) {
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
        _ = ctrl_c => {
            abort_handles.iter().for_each(|handle| handle.abort());
        },
        _ = terminate => {
            abort_handles.iter().for_each(|handle| handle.abort());
        },
    }
}

fn parse_cookie_key(cookie_key: &str) -> Key {
    let key: Vec<u8> = cookie_key[1..cookie_key.len() - 1]
        .split(", ")
        .filter_map(|byte| u8::from_str(byte.trim()).ok())
        .collect();

    Key::from(&key)
}
