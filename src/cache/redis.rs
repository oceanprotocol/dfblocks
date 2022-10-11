use redis::{self, Commands};

pub fn get_client() -> redis::Client {
    let redis_host = std::env::var("REDIS_HOST").unwrap_or("redis://redis".to_string());
    redis::Client::open(redis_host).unwrap()
}

pub fn get_cache_blocks(ts: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let client = get_client();
    let mut con = client.get_connection().unwrap();
    let key = format!("blocks_{}", ts);
    let cache: String = con.get(key)?;
    if cache.is_empty() {
        return Err("not found".into());
    }

    let block: u64 = serde_json::from_str(&cache)?;
    Ok(block)
}

pub fn set_cache_blocks(ts: u64, block: u64) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client();
    let mut con = client.get_connection().unwrap();
    let key = format!("blocks_{}", ts);
    let _: () = con.set(key, block).unwrap();
    Ok(())
}
