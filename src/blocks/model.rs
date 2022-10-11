use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Blocks {
    pub ts_start: u64,
    pub ts_end: u64,
    pub blocks: Vec<u64>,
}
