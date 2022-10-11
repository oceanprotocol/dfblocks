pub fn get_blocks_by_chain(chain_id: u32) -> Vec<u32> {
    let mut blocks = Vec::new();
    for i in 0..10 {
        blocks.push(i);
    }
    blocks
}
