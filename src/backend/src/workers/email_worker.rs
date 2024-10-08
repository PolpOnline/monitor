use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use color_eyre::Result;
use humanize_duration::{prelude::*, Truncate};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncTransport,
    Message, Tokio1Executor,
};
use sidekiq::Worker;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    web::utils::{time::approx_expected_timestamp, time_conversions::pg_interval_to_duration},
    SITE_URL,
};

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

        // Finalize by setting the down_sent_email flag to true for all systems that
        // have been sent an email
        sqlx::query!(
            // language=PostgreSQL
            r#"
            UPDATE system
            SET down_sent_email = TRUE
            WHERE id = ANY($1)
            "#,
            down_services
                .iter()
                .map(|email_data| email_data.system_id)
                .collect::<Vec<_>>()
                .as_slice()
        );

        Ok(())
    }
}

//noinspection HtmlUnknownTarget
fn compose_email(email_data: EmailData) -> GenericResult<Message> {
    info!(
        "Scheduled task: Composing email for the system {} (id {}, user email {}, down since {})",
        email_data.system_name,
        email_data.system_id,
        email_data.user_email,
        email_data.utc_timestamp
    );

    let local_timestamp = email_data.utc_timestamp.with_timezone(&email_data.timezone);
    let down_after = email_data.down_after.human(Truncate::Minute);

    let message = Message::builder()
        .from("Monitor Mailer <monitor@polp.online>".parse()?)
        .reply_to("Monitor Mailer <monitor@polp.online>".parse()?)
        .to(format!("User <{}>", email_data.user_email)
            .as_str()
            .parse()?)
        .subject(format!("Service {} is down", email_data.system_name).as_str())
        .header(ContentType::TEXT_HTML)
        .body(format!(
            // language=HTML
            r#"
                <p>
                  Service {} (system id {}) is down since
                  <time datetime="{}">
                  {}
                  </time>.
                  <br />
                  It was supposed to be up after {}.
                  <br />
                  Check its status now at
                  <a href="{}">{}</a>.
                </p>
                "#,
            email_data.system_name,
            email_data.system_id,
            email_data.utc_timestamp.to_rfc3339(),
            local_timestamp,
            down_after,
            SITE_URL.as_str(),
            SITE_URL.as_str()
        ))?;

    Ok(message)
}

#[derive(Debug)]
pub struct EmailData {
    pub system_id: Uuid,
    pub utc_timestamp: DateTime<Utc>,
    pub down_after: Duration,
    pub system_name: String,
    pub user_email: String,
    pub timezone: Tz,
}

async fn query_down_services(db: &PgPool) -> GenericResult<Vec<EmailData>> {
    // Query all systems that are down for longer than the down_after interval for
    // which an email has not been sent
    let rows = sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT s.id AS system_id,
               s.name AS system_name,
               s.starts_at AS system_starts_at,
               s.down_after,
               u.email AS user_email,
               u.timezone AS user_timezone,
               s.frequency,
               latest_ping.timestamp
        FROM system s
            JOIN "user" u ON s.user_id = u.id
        LEFT JOIN LATERAL (
            SELECT p.timestamp
            FROM ping p
            WHERE p.system_id = s.id
            ORDER BY p.timestamp DESC
            LIMIT 1
        ) latest_ping ON TRUE
        WHERE (NOW() - latest_ping.timestamp) > s.down_after
          AND s.deleted = FALSE
          AND s.down_sent_email = FALSE;
        "#
    )
    .fetch_all(db)
    .await?;

    let rows = rows
        .into_iter()
        .filter_map(|row| {
            let user_timezone = match Tz::from_str(&row.user_timezone) {
                Ok(tz) => tz,
                Err(e) => {
                    error!(
                        "Scheduled task: Error parsing timezone: {}, user email is {}",
                        e, row.user_email
                    );
                    return None;
                }
            };

            let frequency = pg_interval_to_duration(row.frequency);

            let precedent_timestamp =
                approx_expected_timestamp(row.timestamp, frequency, row.system_starts_at)
                    .map_err(|e| {
                        error!(
                            "Scheduled task: Error calculating precedent timestamp: {}",
                            e
                        );
                    })
                    .ok()?;

            let utc_timestamp = precedent_timestamp.and_utc();

            Some(EmailData {
                system_id: row.system_id,
                utc_timestamp: utc_timestamp + frequency,
                down_after: pg_interval_to_duration(row.down_after),
                system_name: row.system_name,
                user_email: row.user_email,
                timezone: user_timezone,
            })
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
