//! `Timestamp` conversions. Compiled only with `--features perf-chrono`.

#![cfg(feature = "perf-chrono")]

use vim_rs::perf::Timestamp;

#[test]
fn timestamp_round_trips_through_chrono_utc() {
    let stamp = Timestamp::parse_rfc3339("2026-10-24T10:48:20Z").unwrap();
    let utc: chrono::DateTime<chrono::Utc> = stamp.into();
    assert_eq!(utc.to_rfc3339(), "2026-10-24T10:48:20+00:00");
    let back = Timestamp::from(utc);
    assert_eq!(back, stamp);
}
