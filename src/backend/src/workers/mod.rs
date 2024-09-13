use color_eyre::Result;
use sidekiq::{periodic, Processor};
use sqlx::PgPool;

use crate::workers::email_worker::{EmailWorker, SmtpClient};

pub(crate) mod email_worker;

pub async fn register_workers(
    p: &mut Processor,
    db: PgPool,
    smtp_client: SmtpClient,
) -> Result<()> {
    // Add a new periodic job, every 15 minutes
    periodic::builder("0 */15 * * * *")?
        .name("Check and send emails for down services")
        .queue("down_emails")
        .register(p, EmailWorker::new(db, smtp_client))
        .await?;

    Ok(())
}
