use async_trait::async_trait;
use color_eyre::Result;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncTransport,
    Message, Tokio1Executor,
};
use sidekiq::Worker;
use sqlx::PgPool;
use time::{Duration, PrimitiveDateTime};
use tracing::{error, info};
use uuid::Uuid;

use crate::web::utils::time_conversions::pg_interval_to_duration;

pub type SmtpClient = lettre::AsyncSmtpTransport<Tokio1Executor>;

#[derive(Clone)]
pub struct EmailWorker {
    db: PgPool,
    smtp_client: SmtpClient,
}

impl EmailWorker {
    pub fn new(db: PgPool, smtp_client: SmtpClient) -> Self {
        Self { db, smtp_client }
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type GenericResult<T> = Result<T, GenericError>;

#[async_trait]
impl Worker<()> for EmailWorker {
    async fn perform(&self, _args: ()) -> sidekiq::Result<()> {
        info!("Scheduled task: checking and sending emails for down services");

        let down_services = query_down_services(&self.db).await?;

        let emails = down_services
            .into_iter()
            .map(compose_email)
            .collect::<Vec<_>>();

        let emails = emails.into_iter().filter_map(|email| match email {
            Ok(email) => Some(email),
            Err(e) => {
                error!("Scheduled task: Error composing email: {}", e);
                None
            }
        });

        for email in emails {
            self.smtp_client.send(email).await.map_err(|e| {
                error!("Scheduled task: Error sending email: {}", e);
                GenericError::from(e)
            })?;
        }

        Ok(())
    }
}

fn compose_email(email_data: EmailData) -> GenericResult<Message> {
    info!(
        "Scheduled task: Composing email for system {} (id {}, user email {}, down since {})",
        email_data.system_name, email_data.system_id, email_data.user_email, email_data.timestamp
    );

    let message = Message::builder()
        .from("Monitor Mailer <monitor@polp.online>".parse()?)
        .reply_to("Monitor Mailer <monitor@polp.online>".parse()?)
        .to(format!("User <{}>", email_data.user_email)
            .as_str()
            .parse()?)
        .subject(format!("Service {} is down", email_data.system_name).as_str())
        .header(ContentType::TEXT_HTML)
        .body(
            format!(
                // language=HTML
                "
                <p>
                  Service {} (system id {}) is down since
                  <time datetime="{}">
                  {} UTC
                  </time>.
                 <br>
                  It was supposed to be up after {}.
                </p>
                ",
                email_data.system_name,
                email_data.system_id,
                email_data.timestamp,
                email_data.timestamp,
                email_data.down_after
            )
            .as_bytes()
            .to_vec(),
        )?;

    Ok(message)
}

#[derive(Debug)]
pub struct EmailData {
    pub system_id: Uuid,
    pub timestamp: PrimitiveDateTime,
    pub down_after: Duration,
    pub system_name: String,
    pub user_email: String,
}

async fn query_down_services(db: &PgPool) -> GenericResult<Vec<EmailData>> {
    let rows = sqlx::query!(
        // language=PostgreSQL
        r#"
        WITH ranked_pings AS (
            SELECT p.id,
                   p.system_id,
                   p.timestamp,
                   s.down_after,
                   s.down_sent_email,
                   s.name                                                                 AS system_name,
                   u.email                                                                AS user_email,
                   ROW_NUMBER() OVER (PARTITION BY p.system_id ORDER BY p.timestamp DESC) AS rn
            FROM ping p
                     JOIN
                 system s ON p.system_id = s.id
                     JOIN
                 "user" u ON s.user_id = u.id
            WHERE NOT EXISTS (
                              SELECT 1
                              FROM ping
                              WHERE ping.system_id = s.id
                                AND ping.timestamp >= NOW() - s.down_after
                              )       
              AND s.down_sent_email = FALSE
              AND s.deleted = FALSE
        ),
        
        updated_systems AS (
            UPDATE system
            SET down_sent_email = TRUE
            FROM ranked_pings rp
            WHERE system.id = rp.system_id
                AND rp.rn = 1
                AND system.deleted = FALSE
            RETURNING rp.system_id, rp.timestamp, rp.down_after, rp.system_name, rp.user_email, system.frequency
        )
        
        SELECT *
        FROM updated_systems;
        "#
    )
    .fetch_all(db)
    .await?;

    let rows = rows
        .into_iter()
        .map(|row| EmailData {
            system_id: row.system_id,
            timestamp: row.timestamp + pg_interval_to_duration(row.frequency),
            down_after: pg_interval_to_duration(row.down_after),
            system_name: row.system_name,
            user_email: row.user_email,
        })
        .collect();

    Ok(rows)
}

pub fn init_smtp_client() -> Result<SmtpClient> {
    let host = std::env::var("EMAIL_HOST")?;
    let username = std::env::var("EMAIL_USERNAME")?;
    let password = std::env::var("EMAIL_PASSWORD")?;

    let creds = Credentials::new(username, password);

    let client = SmtpClient::relay(&host)?.credentials(creds).build();

    Ok(client)
}
