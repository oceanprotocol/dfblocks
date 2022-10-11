use crate::time::thu::{get_thursday_timestamp_now, get_thursday_timestamp_ts};

use super::{calc::getnumbers, model::Blocks};

pub async fn get_blocks_by_chain(chain_id: u64) -> Result<Blocks, Box<dyn std::error::Error>> {
    let ts = get_thursday_timestamp_now();
    getnumbers(chain_id, 50 as u64, ts).await
}

pub async fn get_blocks_by_chain_and_ts(
    chain_id: u64,
    ts: u64,
) -> Result<Blocks, Box<dyn std::error::Error>> {
    let ts = get_thursday_timestamp_ts(ts)?;
    getnumbers(chain_id, 50 as u64, ts).await
}
