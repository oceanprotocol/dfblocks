use super::super::cache::redis;
use super::model::Blocks;
use super::{api::get_block_number_from_timestamp, random::random_choose};

pub async fn getnumbers(
    chain_id: u64,
    samples: u64,
    timestamp: (u64, u64),
) -> Result<Blocks, Box<dyn std::error::Error>> {
    let (start_ts, end_ts) = timestamp;

    // check cache
    let cache = redis::get_cache(start_ts);
    if cache.is_ok() {
        return cache;
    }

    let start = get_block_number_from_timestamp(chain_id, start_ts).await?;
    let end = get_block_number_from_timestamp(chain_id, end_ts).await?;

    let numbers = random_choose(start, end, samples);
    let response = Blocks {
        start_ts,
        end_ts,
        blocks: numbers,
    };

    // add to cache
    redis::set_cache(start_ts, &response)?;
    Ok(response)
}
