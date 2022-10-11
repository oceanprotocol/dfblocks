use chrono::{self, DateTime, Datelike, Utc};

fn _get_thursday_timestamp(now: DateTime<Utc>) -> (u64, u64) {
    let now = now.date().and_hms(0, 0, 0);
    let weekday = now.date().weekday().num_days_from_monday();
    let end_ts: u64;
    if weekday == 4 {
        end_ts = now.timestamp() as u64;
    } else {
        let last_thursday = now - chrono::Duration::days(weekday as i64 + 4);
        end_ts = last_thursday.timestamp() as u64;
    }

    let start_ts = end_ts - 604800;
    return (start_ts, end_ts);
}

pub fn get_thursday_timestamp_now() -> (u64, u64) {
    let now = Utc::now();
    _get_thursday_timestamp(now)
}

pub fn get_thursday_timestamp(timestamp: u64) -> (u64, u64) {
    let now = DateTime::from_utc(
        chrono::NaiveDateTime::from_timestamp(timestamp as i64, 0),
        Utc,
    );
    _get_thursday_timestamp(now)
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_thursday_timestamp() {
        let friday = chrono::DateTime::parse_from_rfc3339("2021-12-31T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let (start_ts, end_ts) = _get_thursday_timestamp(friday);
        assert_eq!(start_ts, 1640304000);
        assert_eq!(end_ts, 1640908800);
    }
}
