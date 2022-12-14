use super::super::cache::redis;
use serde_derive::{Deserialize, Serialize};

static API_URL: &'static str = "https://blockfinder.snapshot.org/graphql";

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    network: String,
    number: u64,
}

pub async fn get_block_number_from_timestamp(
    chain_id: u64,
    timestamp: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    let cache = redis::get_cache_blocks(timestamp, chain_id);
    if cache.is_ok() {
        return cache;
    }

    let query = format!(
        r#"
        {{"query":"query {{
            blocks (where: {{ ts: {}, network_in: [\"{}\"] }}) {{
              network
              number
            }}
        }}"}}
        "#,
        timestamp, chain_id
    );

    // replace \n with empty string
    let query = query.replace('\n', "");
    let client = reqwest::Client::new();
    let res = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .body(query)
        .send()
        .await?;

    let body = res.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&body)?;
    let resp = api_response.data.blocks[0].number;

    redis::set_cache_blocks(timestamp, resp, chain_id)?;
    Ok(resp)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block_number_from_timestamp_1() {
        let block_number = get_block_number_from_timestamp(1, 1640000000)
            .await
            .unwrap();
        assert_eq!(block_number, 13841761);
    }
    #[tokio::test]
    async fn test_get_block_number_from_timestamp_56() {
        let block_number = get_block_number_from_timestamp(56, 1640000000)
            .await
            .unwrap();
        assert_eq!(block_number, 13639035);
    }
    #[tokio::test]
    async fn test_get_block_number_from_timestamp_137() {
        let block_number = get_block_number_from_timestamp(137, 1640000000)
            .await
            .unwrap();
        assert_eq!(block_number, 22747386);
    }
    #[tokio::test]
    async fn test_get_block_number_from_timestamp_246() {
        let block_number = get_block_number_from_timestamp(246, 1640000000)
            .await
            .unwrap();
        assert_eq!(block_number, 15429205);
    }
    #[tokio::test]
    async fn test_get_block_number_from_timestamp_1285() {
        let block_number = get_block_number_from_timestamp(1285, 1640000000)
            .await
            .unwrap();
        assert_eq!(block_number, 1126445);
    }
}
