fn get_thursday_timestamp(now: DateTime<Utc>) -> (u64, u64) {
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
