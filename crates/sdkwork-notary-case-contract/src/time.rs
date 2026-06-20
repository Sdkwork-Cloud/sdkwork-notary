use sdkwork_utils_rust::{format_datetime, now};

pub fn now_iso8601() -> String {
    format_datetime(now(), Some("%Y-%m-%dT%H:%M:%SZ"))
}

pub fn now_compact_date() -> String {
    format_datetime(now(), Some("%Y%m%d"))
}
