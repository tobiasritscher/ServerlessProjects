use actix_web::{web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    timestamp: Option<chrono::DateTime<chrono::Utc>>,
    id: Option<String>,
    region_id: Option<String>,
    data: Option<String>,
    device_data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
#[repr(transparent)]
struct Webhook {
    data: Vec<Info>,
}

/// extract `Info` using serde
#[actix_web::post("/webhook")]
async fn webhook(_info: web::Json<Info>) -> Result<impl Responder> {
    // TODO:
    Ok("todo")
}

#[actix_web::get("/stats")]
async fn stats() -> Result<impl Responder> {
    // TODO:
    Ok("todo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(webhook).service(stats))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
