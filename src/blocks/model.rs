use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blocks {
    pub start_ts: u64,
    pub end_ts: u64,
    pub blocks: Vec<u64>,
}
