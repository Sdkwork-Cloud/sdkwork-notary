use chrono::Utc;

pub fn now_iso8601() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn now_compact_date() -> String {
    Utc::now().format("%Y%m%d").to_string()
}
