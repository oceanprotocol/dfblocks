use crate::time::thu::{get_thursday_timestamp_now, get_thursday_timestamp_ts};

use super::{calc::getnumbers, model::Blocks};

pub async fn get_blocks_by_chain(
    chain_id: u64,
    samples: u64,
    ts: u64,
) -> Result<Blocks, Box<dyn std::error::Error>> {
    if samples > 500 {
        return Err("samples can't be greater than 500".into());
    }
    let timestamps: (u64, u64);
    if ts == 0 {
        timestamps = get_thursday_timestamp_now();
    } else {
        timestamps = get_thursday_timestamp_ts(ts)?;
    }
    getnumbers(chain_id, samples, timestamps).await
}
