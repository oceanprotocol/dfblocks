use super::{calc::getnumbers, model::Blocks};

pub async fn get_blocks_by_chain(chain_id: u64) -> Result<Blocks, Box<dyn std::error::Error>> {
    getnumbers(chain_id, 50 as u64).await
}
