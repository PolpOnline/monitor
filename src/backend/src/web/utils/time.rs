use chrono::{Duration, NaiveDateTime, Utc};
use thiserror::Error;

pub fn naive_datetime_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[derive(Error, Debug)]
pub enum ApproxError {
    #[error("Invalid frequency")]
    InvalidFrequency,
}

/// Rounds the timestamp to the closest expected timestamp given a frequency and
/// a start_at timestamp
pub fn approx_expected_timestamp(
    timestamp_to_approx: NaiveDateTime,
    frequency: Duration,
    starts_at: NaiveDateTime,
) -> Result<NaiveDateTime, ApproxError> {
    if frequency.is_zero() {
        return Err(ApproxError::InvalidFrequency);
    }

    let duration_since_start = timestamp_to_approx - starts_at;
    let intervals = duration_since_start.num_seconds() / frequency.num_seconds();
    let expected_timestamp = starts_at + frequency * (intervals as i32);

    Ok(expected_timestamp)
}

mod test {
    #[test]
    fn test_approx_expected_timestamp() {
        use super::*;

        let start_at = naive_datetime_now();
        let frequency = Duration::minutes(30);

        // Timestamp is sufficiently near the expected timestamp
        let timestamp = start_at + Duration::minutes(2);
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at
        );

        // Timestamp is too far from the expected timestamp, approximates to the next
        // one
        let timestamp = start_at + Duration::minutes(32);
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + Duration::minutes(30)
        );

        // The function should only floor the timestamp
        let timestamp = start_at + Duration::minutes(59);
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + Duration::minutes(30)
        );

        // Another case where the function should only floor the timestamp
        let timestamp = start_at + Duration::minutes(60);
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + Duration::minutes(60)
        );

        // Another case where the function should only floor the timestamp
        let timestamp = start_at + Duration::minutes(61);
        assert_eq!(
            approx_expected_timestamp(timestamp, frequency, start_at).unwrap(),
            start_at + Duration::minutes(60)
        );

        // Negative case: frequency is zero
        let timestamp = start_at + Duration::minutes(15);
        assert!(approx_expected_timestamp(timestamp, Duration::seconds(0), start_at).is_err());
    }
}
