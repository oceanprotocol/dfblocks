use chrono::{self, DateTime, Datelike, Utc};

use super::api::get_block_number_from_timestamp;

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

pub async fn getnumbers(
    chain_id: u64,
    samples: u64,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let (start_ts, end_ts) = get_thursday_timestamp(now);
    let start = get_block_number_from_timestamp(chain_id, start_ts).await?;
    let end = get_block_number_from_timestamp(chain_id, end_ts).await?;

    let mut numbers = Vec::new();
    let step = (end - start) / samples;
    for i in 0..samples {
        numbers.push(start + step * i);
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_thursday_timestamp() {
        let friday = chrono::DateTime::parse_from_rfc3339("2021-12-31T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let (start_ts, end_ts) = get_thursday_timestamp(friday);
        assert_eq!(start_ts, 1640304000);
        assert_eq!(end_ts, 1640908800);
    }
}
