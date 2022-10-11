use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dfblocks::blocks::blocks::{get_blocks_by_chain, get_blocks_by_chain_and_ts};
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ApiRequest {
    chain_id: u64,
    timestamp: u64,
}

#[post("/blocks")]
async fn handler_get_blocks_by_ts(req_body: web::Json<ApiRequest>) -> impl Responder {
    let chain_id = req_body.chain_id;
    let timestamp = req_body.timestamp;
    let response = get_blocks_by_chain_and_ts(chain_id, timestamp).await;

    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/blocks/{chainId}")]
async fn handler_get_blocks(path: web::Path<u64>) -> impl Responder {
    let chain_id = path.into_inner();
    let blocks = get_blocks_by_chain(chain_id).await;
    match blocks {
        Ok(blocks) => HttpResponse::Ok().json(blocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(handler_get_blocks)
            .service(handler_get_blocks_by_ts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
