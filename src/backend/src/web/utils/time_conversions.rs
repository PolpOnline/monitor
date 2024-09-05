use time::{ext::NumericalDuration, Duration, OffsetDateTime, PrimitiveDateTime};

#[inline]
pub fn offset_to_primitive_date_time(offset_date_time: OffsetDateTime) -> PrimitiveDateTime {
    PrimitiveDateTime::new(offset_date_time.date(), offset_date_time.time())
}

#[inline]
pub fn primitive_to_offset_date_time(primitive_date_time: PrimitiveDateTime) -> OffsetDateTime {
    OffsetDateTime::new_utc(primitive_date_time.date(), primitive_date_time.time())
}

pub fn from_pg_interval_to_duration(interval: sqlx::postgres::types::PgInterval) -> Duration {
    let months = interval.months as i64;
    let days = interval.days as i64;
    let microseconds = interval.microseconds;

    let micros =
        months * 30 * 24 * 60 * 60 * 1_000_000 + days * 24 * 60 * 60 * 1_000_000 + microseconds;

    micros.microseconds()
}
