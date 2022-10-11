use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Blocks {
    pub start_ts: u64,
    pub end_ts: u64,
    pub blocks: Vec<u64>,
}
