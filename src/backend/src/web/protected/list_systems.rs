use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
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
#[ts(export)]
pub enum Status {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "untracked")]
    Untracked,
}

const LIMIT_SYSTEM_REQUEST: i32 = 100;

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

    let db_systems = match sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT * FROM system WHERE user_id = $1 AND deleted = false ORDER BY starts_at
        "#,
        user.id
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut systems: Vec<SystemData> = Vec::with_capacity(db_systems.len());

    for system in db_systems {
        let db_instants = match sqlx::query_as!(
            PingRecord,
            // language=PostgreSQL
            r#"
            SELECT * FROM ping WHERE system_id = $1 ORDER BY timestamp DESC LIMIT $2
            "#,
            system.id,
            list_size as i64
        )
        .fetch_all(&auth_session.backend.db)
        .await
        {
            Ok(r) => r,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let frequency = from_pg_interval_to_duration(system.frequency);

        let instants =
            from_ping_records_to_instants(db_instants, frequency, system.starts_at, list_size);

        let system_data = SystemData {
            id: system.id,
            name: system.name,
            instants,
            frequency: (frequency.as_seconds_f64().round() as u32) / 60,
            starts_at: primitive_to_offset_date_time(system.starts_at),
        };

        systems.push(system_data);
    }

    Json(ListSystemsResponse { systems }).into_response()
}

#[derive(Debug)]
pub struct PingRecord {
    #[allow(dead_code)]
    id: i32,
    #[allow(dead_code)]
    system_id: String,
    timestamp: PrimitiveDateTime,
}

// here we convert the records from the ping table to a vector of Instant
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
