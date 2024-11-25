use std::str::FromStr;

use axum::middleware;
use axum_login::{
    tower_sessions::{Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::StatusCode;
use sidekiq::{periodic, Processor, RedisConnectionManager};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{signal, task::AbortHandle};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer, trace::TraceLayer,
};
use tower_sessions::cookie::Key;
use tower_sessions_redis_store::{
    fred::{
        prelude::{ClientLike, RedisConfig as RedisFredConfig, RedisPool as RedisFredPool},
        types::ReconnectPolicy,
    },
    RedisStore,
};
use tracing::info;

use crate::{
    custom_login_required,
    middleware::{set_cache_control::set_cache_control, set_user_info::set_user_info},
    users::LoginBackend,
    web::{auth, protected, public},
    workers::{
        email_worker::{init_smtp_client, SmtpClient},
        register_workers,
    },
    PRODUCTION,
};

type RedisLibPool = bb8::Pool<RedisConnectionManager>;

pub struct App {
    db: PgPool,
    redis_lib: RedisLibPool,
    redis_fred: RedisFredPool,
    smtp_client: SmtpClient,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let db = Self::setup_db().await?;
        let redis_lib = Self::setup_redis_lib().await?;
        let redis_fred = Self::setup_redis_fred().await?;
        let smtp_client = init_smtp_client()?;

        Ok(Self {
            db,
            redis_lib,
            redis_fred,
            smtp_client,
        })
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = RedisStore::new(self.redis_fred.clone());

        let processor =
            Self::init_workers(self.redis_lib.clone(), self.db.clone(), self.smtp_client).await?;

        let worker_task = tokio::task::spawn(Self::start_workers(processor));

        // Generate a cryptographic key to sign the session cookie.
        let key = &std::env::var("COOKIE_KEY")?;
        let key = parse_cookie_key(key);

        let session_layer = SessionManagerLayer::new(session_store)
            .with_name("monitor_id")
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(
                tower_sessions::cookie::time::Duration::days(7),
            ))
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
            .layer(middleware::from_fn(set_user_info))
            .layer(auth_layer)
            .layer(middleware::from_fn(set_cache_control))
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(DecompressionLayer::new());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Axum: Listening on {}", listener.local_addr()?);

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(vec![worker_task.abort_handle()]))
            .await?;

        futures::future::join_all(vec![worker_task]).await;

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

    async fn setup_redis_lib() -> color_eyre::Result<RedisLibPool> {
        info!("Redis Lib: Connecting to Redis (to manage workers)...");

        let db_num = 1u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);
        let manager = RedisConnectionManager::new(redis_url)?;
        let redis = bb8::Pool::builder().build(manager).await?;

        info!("Redis Lib: Connected to Redis (to manage workers)");

        Ok(redis)
    }

    async fn setup_redis_fred() -> color_eyre::Result<RedisFredPool> {
        info!("Redis Fred: Connecting to Redis (to manage sessions)...");

        let db_num = 0u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);

        let config = RedisFredConfig::from_url(&redis_url)?;

        let pool = RedisFredPool::new(config, None, None, Some(ReconnectPolicy::default()), 6)?;

        pool.init().await?;

        info!("Redis Fred: Connected to Redis (to manage sessions)");

        Ok(pool)
    }

    async fn start_workers(p: Processor) -> color_eyre::Result<()> {
        info!(
            "Sidekiq: Workers started in {} mode",
            if *PRODUCTION {
                "production"
            } else {
                "development"
            }
        );

        // Start the server
        p.run().await;

        Ok(())
    }

    async fn init_workers(
        redis: RedisLibPool,
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
