use time::{OffsetDateTime, PrimitiveDateTime};

pub fn from_offset_date_time_to_primitive_date_time(
    offset_date_time: OffsetDateTime,
) -> PrimitiveDateTime {
    PrimitiveDateTime::new(offset_date_time.date(), offset_date_time.time())
}
