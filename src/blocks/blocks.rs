use crate::time::thu::{get_thursday_timestamp, get_thursday_timestamp_now};

use super::{calc::getnumbers, model::Blocks};

pub async fn get_blocks_by_chain(chain_id: u64) -> Result<Blocks, Box<dyn std::error::Error>> {
    let ts = get_thursday_timestamp_now();
    getnumbers(chain_id, 50 as u64, ts).await
}

pub async fn get_blocks_by_chain_and_ts(
    chain_id: u64,
    ts: u64,
) -> Result<Blocks, Box<dyn std::error::Error>> {
    let ts = get_thursday_timestamp(ts);
    getnumbers(chain_id, 50 as u64, ts).await
}
