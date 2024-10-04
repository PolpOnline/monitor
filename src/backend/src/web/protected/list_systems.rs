use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgInterval, PgPool};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    users::AuthSession,
    web::utils::{
        time::{approx_expected_timestamp, naive_datetime_now},
        time_conversions::pg_interval_to_duration,
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
    #[ts(type = "string")]
    starts_at: DateTime<Utc>,
    visibility: Visibility,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Instant {
    status: Status,
    #[ts(type = "string | null")]
    timestamp: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    expected_timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Status {
    Up,
    Down,
    Untracked,
}

pub const LIMIT_SYSTEM_REQUEST: i64 = 100;

#[derive(Debug, Serialize, Deserialize, Clone, TS, sqlx::Type)]
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
    pub starts_at: NaiveDateTime,
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
    timestamp: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListSystemsQuery {
    pub page: i64,
    pub list_size: i64,
}

pub async fn list_systems(
    auth_session: AuthSession,
    Query(query): Query<ListSystemsQuery>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if query.list_size > LIMIT_SYSTEM_REQUEST {
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
        let system_data = match SystemData::fetch_from_db(
            &auth_session.backend.db,
            query.list_size,
            query.page,
            db_system,
        )
        .await
        {
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
        list_size: i64,
        page: i64,
        db_system: SystemRecord,
    ) -> Result<Self, Response> {
        let frequency = pg_interval_to_duration(db_system.frequency);

        let now = approx_expected_timestamp(naive_datetime_now(), frequency, db_system.starts_at)
            .unwrap();

        let nearest_datetime = now - (frequency * (page * list_size) as i32);
        let furthest_datetime = nearest_datetime - (frequency * list_size as i32);

        let nearest_datetime_max_expected = nearest_datetime + frequency;
        let furthest_datetime_min_expected = furthest_datetime + frequency;

        let db_instants = match sqlx::query_as!(
            PingRecord,
            // language=PostgreSQL
            r#"
            SELECT * FROM ping WHERE system_id = $1
                                AND timestamp < $2
                                AND timestamp > $3
                               ORDER BY timestamp DESC
            "#,
            db_system.id,
            nearest_datetime_max_expected,
            furthest_datetime_min_expected,
        )
        .fetch_all(pg_pool)
        .await
        {
            Ok(r) => r,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };

        let instants = Self::from_ping_records_to_instants(
            db_instants,
            frequency,
            db_system.starts_at,
            nearest_datetime,
            furthest_datetime,
            list_size,
        )?;

        Ok(SystemData {
            id: db_system.id,
            name: db_system.name,
            instants,
            frequency: frequency.num_seconds() as u32 / 60,
            starts_at: db_system.starts_at.and_utc(),
            visibility: db_system.visibility,
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
        starts_at: NaiveDateTime,
        mut nearest_datetime: NaiveDateTime,
        furthest_datetime: NaiveDateTime,
        list_size: i64,
    ) -> Result<Vec<Instant>, Response> {
        // Hashmap that contains the key as the expected timestamp and the value as the
        // actual timestamp
        let hashmap = ping_records.iter().fold(HashMap::new(), |mut acc, t| {
            acc.insert(
                approx_expected_timestamp(t.timestamp, frequency, starts_at).unwrap(),
                t.timestamp,
            );
            acc
        });

        let mut instants = Vec::with_capacity(list_size as usize);

        while nearest_datetime > furthest_datetime {
            let instant = match hashmap.get(&nearest_datetime) {
                Some(status) => Instant {
                    status: Status::Up,
                    timestamp: Some(status.and_utc()),
                    expected_timestamp: nearest_datetime.and_utc(),
                },
                None => {
                    let status = if nearest_datetime > starts_at {
                        Status::Down
                    } else {
                        Status::Untracked
                    };

                    Instant {
                        status,
                        timestamp: None,
                        expected_timestamp: nearest_datetime.and_utc(),
                    }
                }
            };

            instants.push(instant);

            nearest_datetime -= frequency;
        }

        instants.reverse();

        Ok(instants)
    }
}
