use super::super::blocks::model::Blocks;
use redis::{self, Commands};

pub fn get_client() -> redis::Client {
    redis::Client::open("redis://localhost/").unwrap()
}

pub fn get_cache(start_ts: u64) -> Result<Blocks, Box<dyn std::error::Error>> {
    let client = get_client();
    let mut con = client.get_connection().unwrap();
    let key = format!("blocks_{}", start_ts);
    let cache: String = con.get(key).unwrap();
    if cache.is_empty() {
        return Err("not found".into());
    }

    let blocks: Blocks = serde_json::from_str(&cache)?;
    Ok(blocks)
}

pub fn set_cache(start_ts: u64, blocks: &Blocks) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client();
    let mut con = client.get_connection().unwrap();
    let key = format!("blocks_{}", start_ts);
    let cache = serde_json::to_string(blocks)?;
    let _: () = con.set(key, cache).unwrap();
    Ok(())
}
