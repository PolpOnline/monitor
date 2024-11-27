use sidekiq::{periodic, Processor};
use sqlx::PgPool;
use tracing::info;

use crate::{
    app::{redis::RedisLibPool, App},
    workers::{email_worker::SmtpClient, register_workers},
    PRODUCTION,
};

impl App {
    pub(super) async fn start_workers(p: Processor) -> color_eyre::Result<()> {
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

    pub(super) async fn init_workers(
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
