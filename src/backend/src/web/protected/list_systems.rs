use std::collections::HashMap;

use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use ts_rs::TS;

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
    name: String,
    instants: Vec<Instant>,
    /// Frequency in minutes
    frequency: u32,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Instant {
    status: Status,
    #[serde(with = "time::serde::iso8601::option")]
    #[ts(type = "Date | null")]
    timestamp: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601")]
    #[ts(type = "Date")]
    expected_timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub enum Status {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}

// TODO: For now we are hardcoding the limit of instants to 30
const LIMIT_INSTANTS: i64 = 30;

pub async fn list_systems(auth_session: AuthSession) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db_systems = match sqlx::query!("SELECT * FROM system WHERE user_id = $1", user.id)
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
            r#"
            SELECT * FROM ping WHERE system_id = $1 ORDER BY timestamp ASC LIMIT $2
            "#,
            system.id,
            LIMIT_INSTANTS
        )
        .fetch_all(&auth_session.backend.db)
        .await
        {
            Ok(r) => r,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let frequency = from_pg_interval_to_duration(system.frequency);

        let instants = from_ping_records_to_instants(
            db_instants,
            frequency,
            system.starts_at,
            LIMIT_INSTANTS as i32,
        );

        let system_data = SystemData {
            name: system.name,
            instants,
            frequency: (frequency.as_seconds_f64().round() as u32) / 60,
        };

        systems.push(system_data);
    }

    Json(ListSystemsResponse { systems }).into_response()
}

#[derive(Debug)]
struct PingRecord {
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
    while expected_timestamp >= how_many_datetime {
        let instant = match hashmap.get(&expected_timestamp) {
            Some(status) => Instant {
                status: Status::Up,
                timestamp: Some(primitive_to_offset_date_time(*status)),
                expected_timestamp: primitive_to_offset_date_time(expected_timestamp),
            },
            None => Instant {
                status: Status::Down,
                timestamp: None,
                expected_timestamp: primitive_to_offset_date_time(expected_timestamp),
            },
        };

        instants.push(instant);

        expected_timestamp -= frequency;
    }

    instants
}
