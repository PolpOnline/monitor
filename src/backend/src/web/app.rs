use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::StatusCode;
use sqlx::{postgres::PgPoolOptions, PgPool};
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::{
    custom_login_required,
    users::Backend,
    web::{auth, protected, public},
};

pub struct App {
    db: PgPool,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let db = Self::setup_db().await?;

        Ok(Self { db })
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
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // Generate a cryptographic key to sign the session cookie.
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(30)))
            .with_signed(key);

        // Auth service.
        //
        // This combines the session layer with our backendOld to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = Backend::new(self.db);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app = protected::router()
            .route_layer(custom_login_required!(
                Backend,
                (StatusCode::UNAUTHORIZED, "You are not logged in.")
            ))
            .merge(auth::router())
            .merge(public::router())
            .layer(auth_layer)
            .layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Listening on {}", listener.local_addr()?);

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        deletion_task.await??;

        Ok(())
    }

    async fn setup_db() -> color_eyre::Result<PgPool> {
        info!("Connecting to the database...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;

        info!("Connected to the database");

        sqlx::migrate!().run(&pool).await?;

        info!("Migrations run");

        Ok(pool)
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
