use thiserror::Error;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

pub fn primitive_datetime_now() -> PrimitiveDateTime {
    let now_odt = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now_odt.date(), now_odt.time())
}

#[derive(Error, Debug)]
pub enum ApproxError {
    #[error("Invalid frequency")]
    InvalidFrequency,
}

/// Rounds the timestamp to the closest expected timestamp given a frequency and
/// a start_at timestamp
pub fn approx_expected_timestamp(
    timestamp_to_approx: PrimitiveDateTime,
    frequency: Duration,
    starts_at: PrimitiveDateTime,
) -> Result<PrimitiveDateTime, ApproxError> {
    if frequency.is_zero() {
        return Err(ApproxError::InvalidFrequency);
    }

    let duration_since_start = timestamp_to_approx - starts_at;
    let intervals = duration_since_start.whole_seconds() / frequency.whole_seconds();
    let expected_timestamp = starts_at + frequency * (intervals as i32);

    Ok(expected_timestamp)
}

mod test {
    #[test]
    fn test_approx_expected_timestamp() {
        use time::ext::NumericalDuration;

        use super::*;

        let start_at = primitive_datetime_now();
        let frequency = 30.minutes();

        // Timestamp is sufficiently near the expected timestamp
        let timestamp = start_at + 2.minutes();
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at
        );

        // Timestamp is too far from the expected timestamp, approximates to the next
        // one
        let timestamp = start_at + 32.minutes();
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + 30.minutes()
        );

        // The function should only floor the timestamp
        let timestamp = start_at + 59.minutes();
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + 30.minutes()
        );

        // Another case where the function should only floor the timestamp
        let timestamp = start_at + 60.minutes();
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + 60.minutes()
        );

        // Another case where the function should only floor the timestamp
        let timestamp = start_at + 61.minutes();
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + 60.minutes()
        );

        // Negative case: frequency is zero
        let timestamp = start_at + 15.minutes();
        assert!(approx_expected_timestamp(timestamp, Duration::ZERO, start_at).is_err());
    }
}
