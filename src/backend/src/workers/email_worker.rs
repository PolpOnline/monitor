use async_trait::async_trait;
use sidekiq::{Result, Worker};
use sqlx::PgPool;
use tracing::info;

#[derive(Clone)]
pub struct EmailWorker {
    db: PgPool,
}

impl EmailWorker {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Worker<()> for EmailWorker {
    async fn perform(&self, _args: ()) -> Result<()> {
        info!("Checking and sending emails for down services");

        Ok(())
    }
}
