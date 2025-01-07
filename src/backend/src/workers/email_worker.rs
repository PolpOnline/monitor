use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use color_eyre::Result;
use humanize_duration::{prelude::*, Truncate};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use rust_i18n::t;
use sidekiq::Worker;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    web::utils::{time::approx_expected_timestamp, time_conversions::pg_interval_to_duration},
    SITE_URL,
};

pub type SmtpClient = AsyncSmtpTransport<Tokio1Executor>;

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

const WAIT_DURATION: Duration = Duration::minutes(2);

#[async_trait]
impl Worker<()> for EmailWorker {
    async fn perform(&self, _args: ()) -> sidekiq::Result<()> {
        info!(
            "Scheduled task: Email started, waiting for {} before sending emails",
            WAIT_DURATION.human(Truncate::Minute)
        );
        tokio::time::sleep(WAIT_DURATION.to_std()?).await;
        info!("Scheduled task: Checking and sending emails for down services");

        let down_services = query_down_services(&self.db).await?;

        let emails = down_services.iter().map(compose_email).collect::<Vec<_>>();

        let emails = emails.into_iter().filter_map(|email| match email {
            Ok(email) => Some(email),
            Err(e) => {
                error!("Scheduled task: Error composing email: {}", e);
                None
            }
        });

        for email in emails {
            match self.smtp_client.send(email).await {
                Ok(_) => {}
                Err(e) => error!("Scheduled task: Error sending email: {}", e),
            }
        }

        let down_ids = down_services
            .iter()
            .map(|email_data| email_data.system_id)
            .collect::<Vec<_>>();

        // Finalize by setting the down_sent_email flag to true for all systems that
        // have been sent an email
        sqlx::query!(
            // language=PostgreSQL
            r#"
            UPDATE system
            SET down_sent_email = TRUE
            WHERE id = ANY($1)
            "#,
            down_ids.as_slice()
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            error!("Scheduled task: Error updating down_sent_email flag: {}", e);
            GenericError::from(e)
        })?;

        Ok(())
    }
}

//noinspection HtmlUnknownTarget
fn compose_email(email_data: &EmailData) -> GenericResult<Message> {
    info!(
        "Scheduled task: Composing email for the system {} (id {}, user email {}, down since {})",
        email_data.system_name,
        email_data.system_id,
        email_data.user_email,
        email_data.utc_timestamp
    );

    let local_timestamp = email_data.utc_timestamp.with_timezone(&email_data.timezone);
    let down_after = email_data.down_after.human(Truncate::Minute);

    let user_locale = email_data.language.as_str();

    let message = Message::builder()
        .from("Monitor Mailer <monitor@polp.online>".parse()?)
        .to(format!("User <{}>", email_data.user_email)
            .as_str()
            .parse()?)
        .subject(t!(
            "email.subject",
            locale = user_locale,
            service_name = email_data.system_name
        ))
        .header(ContentType::TEXT_HTML)
        .body(format!(
            // language=HTML
            r#"
                <p>
                  {}
                  <time datetime="{}">
                  {}
                  </time>.
                  <br />
                  {}
                  <br />
                  {}
                  <a href="{}">{}</a>.
                </p>
            "#,
            t!(
                "email.service_is_down_since",
                locale = user_locale,
                service_name = email_data.system_name
            ),
            email_data.utc_timestamp.to_rfc3339(),
            local_timestamp,
            t!(
                "email.it_was_supposed_to_be_up_after",
                locale = user_locale,
                down_after = down_after
            ),
            t!("email.check_its_status_now_at", locale = user_locale),
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
    pub language: String,
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
               u.language AS user_language,
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
            let down_after = pg_interval_to_duration(row.down_after);

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
                utc_timestamp: utc_timestamp + down_after,
                down_after,
                system_name: row.system_name,
                user_email: row.user_email,
                timezone: user_timezone,
                language: row.user_language,
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
