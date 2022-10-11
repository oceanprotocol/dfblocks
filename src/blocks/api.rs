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
struct Response {}

fn get_block_number_from_timestamp(chain_id: u64, timestamp: u64) -> u64 {
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
    let query = query.replace("\n", "");

    println!("query: {}", query);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .body(query)
        .send()
        .unwrap();

    let body = res.text().unwrap();

    println!("{}", body);

    let response: ApiResponse = serde_json::from_str(&body).unwrap();
    return response.data.blocks[0].number;
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block_number_from_timestamp_1() {
        let block_number = get_block_number_from_timestamp(1, 1640000000);
        assert_eq!(block_number, 13841761);
    }
    #[test]
    fn test_get_block_number_from_timestamp_56() {
        let block_number = get_block_number_from_timestamp(56, 1640000000);
        assert_eq!(block_number, 13639035);
    }
    #[test]
    fn test_get_block_number_from_timestamp_137() {
        let block_number = get_block_number_from_timestamp(137, 1640000000);
        assert_eq!(block_number, 22747386);
    }
    #[test]
    fn test_get_block_number_from_timestamp_246() {
        let block_number = get_block_number_from_timestamp(246, 1640000000);
        assert_eq!(block_number, 15429205);
    }
    #[test]
    fn test_get_block_number_from_timestamp_1285() {
        let block_number = get_block_number_from_timestamp(1285, 1640000000);
        assert_eq!(block_number, 1126445);
    }
}
