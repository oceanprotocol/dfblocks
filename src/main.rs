use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dfblocks::blocks::blocks::get_blocks_by_chain;
use dotenv::dotenv;
#[get("/blocks/{chainId}")]
async fn get_blocks(path: web::Path<u64>) -> impl Responder {
    let chain_id = path.into_inner();
    let blocks = get_blocks_by_chain(chain_id).await;
    match blocks {
        Ok(blocks) => {
            let json = serde_json::to_string(&blocks).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
    // convert blocks to json array using serde
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(get_blocks))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
