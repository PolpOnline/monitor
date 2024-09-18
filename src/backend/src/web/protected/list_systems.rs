use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use sqlx::{postgres::types::PgInterval, PgPool};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    users::AuthSession,
    web::utils::{
        time::{approx_expected_timestamp, primitive_datetime_now},
        time_conversions::{from_pg_interval_to_duration, primitive_to_offset_date_time},
    },
};

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct ListSystemsResponse {
    systems: Vec<SystemData>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct SystemData {
    id: Uuid,
    name: String,
    instants: Vec<Instant>,
    /// Frequency in minutes
    frequency: u32,
    #[serde(with = "time::serde::iso8601")]
    #[ts(type = "string")]
    starts_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Instant {
    status: Status,
    #[serde(with = "time::serde::iso8601::option")]
    #[ts(type = "string | null")]
    timestamp: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601")]
    #[ts(type = "string")]
    expected_timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Status {
    Up,
    Down,
    Untracked,
}

const LIMIT_SYSTEM_REQUEST: i32 = 100;

#[derive(Debug, Serialize, Clone, TS, sqlx::Type)]
#[sqlx(type_name = "visibility", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Visibility {
    Public,
    Private,
}

// Records from the tables
#[derive(Debug, sqlx::FromRow)]
pub struct SystemRecord {
    pub id: Uuid,
    pub name: String,
    pub user_id: i32,
    pub frequency: PgInterval,
    pub starts_at: PrimitiveDateTime,
    pub deleted: bool,
    pub down_after: PgInterval,
    pub down_sent_email: bool,
    pub visibility: Visibility,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PingRecord {
    #[allow(dead_code)]
    id: i32,
    #[allow(dead_code)]
    system_id: String,
    timestamp: PrimitiveDateTime,
}

pub async fn list_systems(
    auth_session: AuthSession,
    Path(list_size): Path<i32>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if list_size > LIMIT_SYSTEM_REQUEST {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let db_systems = match sqlx::query_as!(
        SystemRecord,
        // language=PostgreSQL
        r#"
        SELECT id, name, user_id, frequency, starts_at, deleted, down_after, down_sent_email, visibility AS "visibility: Visibility"
        FROM system WHERE user_id = $1 AND deleted = false ORDER BY starts_at
        "#,
        user.id,
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut systems: Vec<SystemData> = Vec::with_capacity(db_systems.len());

    for db_system in db_systems {
        let system_data =
            match SystemData::fetch_from_db(&auth_session.backend.db, list_size, db_system).await {
                Ok(r) => r,
                Err(s) => return s.into_response(),
            };

        systems.push(system_data);
    }

    Json(ListSystemsResponse { systems }).into_response()
}

impl SystemData {
    pub async fn fetch_from_db(
        pg_pool: &PgPool,
        list_size: i32,
        db_system: SystemRecord,
    ) -> Result<Self, StatusCode> {
        let db_instants = match sqlx::query_as!(
            PingRecord,
            // language=PostgreSQL
            r#"
            SELECT * FROM ping WHERE system_id = $1 ORDER BY timestamp DESC LIMIT $2
            "#,
            db_system.id,
            list_size as i64
        )
        .fetch_all(pg_pool)
        .await
        {
            Ok(r) => r,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let frequency = from_pg_interval_to_duration(db_system.frequency);

        let instants = Self::from_ping_records_to_instants(
            db_instants,
            frequency,
            db_system.starts_at,
            list_size,
        );

        Ok(SystemData {
            id: db_system.id,
            name: db_system.name,
            instants,
            frequency: (frequency.as_seconds_f64().round() as u32) / 60,
            starts_at: primitive_to_offset_date_time(db_system.starts_at),
        })
    }

    // Here we convert the records from the ping table to a vector of Instant
    // objects, we calculate from the starts_at timestamp (system.starts_at) to all
    // the expected times to fill up the "Down" moments
    // uses the approx_expected_timestamp function to calculate the expected
    // timestamp
    fn from_ping_records_to_instants(
        ping_records: Vec<PingRecord>,
        frequency: Duration,
        starts_at: PrimitiveDateTime,
        how_many: i32,
    ) -> Vec<Instant> {
        let expected_timestamp_now =
            approx_expected_timestamp(primitive_datetime_now(), frequency, starts_at).unwrap();

        // Hashmap that contains the key as the expected timestamp and the value as the
        // actual timestamp
        let hashmap = ping_records.iter().fold(HashMap::new(), |mut acc, t| {
            acc.insert(
                approx_expected_timestamp(t.timestamp, frequency, starts_at).unwrap(),
                t.timestamp,
            );
            acc
        });

        let how_many_datetime = expected_timestamp_now - frequency * how_many;

        let mut instants = Vec::with_capacity(how_many as usize);

        let mut expected_timestamp = expected_timestamp_now;
        while expected_timestamp > how_many_datetime {
            let instant = match hashmap.get(&expected_timestamp) {
                Some(status) => Instant {
                    status: Status::Up,
                    timestamp: Some(primitive_to_offset_date_time(*status)),
                    expected_timestamp: primitive_to_offset_date_time(expected_timestamp),
                },
                None => {
                    let status = if expected_timestamp > starts_at {
                        Status::Down
                    } else {
                        Status::Untracked
                    };

                    Instant {
                        status,
                        timestamp: None,
                        expected_timestamp: primitive_to_offset_date_time(expected_timestamp),
                    }
                }
            };

            instants.push(instant);

            expected_timestamp -= frequency;
        }

        instants.reverse();

        instants
    }
}
