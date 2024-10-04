use chrono::Duration;

pub fn pg_interval_to_duration(interval: sqlx::postgres::types::PgInterval) -> Duration {
    let months = interval.months as i64;
    let days = interval.days as i64;
    let microseconds = interval.microseconds;

    let micros =
        months * 30 * 24 * 60 * 60 * 1_000_000 + days * 24 * 60 * 60 * 1_000_000 + microseconds;

    Duration::microseconds(micros)
}
