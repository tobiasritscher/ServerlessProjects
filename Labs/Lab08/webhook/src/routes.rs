use actix_web::{web, Responder, Result};

use crate::stats_storage::{storage, Info};

pub fn get_scope() -> actix_web::Scope {
    web::scope("/").service(webhook).service(stats)
}

/// extract `Info` using serde
#[actix_web::post("/webhook")]
async fn webhook(info: web::Json<Info>) -> Result<impl Responder> {
    // move information direcly, so not to block the response to long
    storage::store(info.0);
    // TODO:
    Ok("todo")
}

#[actix_web::get("/stats")]
async fn stats() -> Result<impl Responder, serde_json::Error> {
    storage::serialized()
}
