use super::calc::getnumbers;

pub async fn get_blocks_by_chain(chain_id: u64) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    getnumbers(chain_id, 50 as u64).await
}
