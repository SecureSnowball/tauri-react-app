use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_timestamp_into_future(hours: u64) -> u64 {
    let hours_into_seconds = hours * 60 * 60;
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is invalid")
        .as_secs()
        + hours_into_seconds
}
