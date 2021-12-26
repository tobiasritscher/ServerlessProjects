use actix_web::{get, post, web, Responder, Result};

use crate::stats_storage::{storage, Info};

pub fn get_scope(path: &str) -> actix_web::Scope {
    web::scope(path).service(webhook).service(stats)
}

/// extract `Info` using serde
#[post("/webhook")]
async fn webhook(info: web::Json<Info>) -> Result<impl Responder> {
    log::debug!("Received webhook data {:?}", info);

    // move information direcly, so not to block the response to long
    storage::store(info.0);
    // TODO:
    Ok("todo")
}

#[get("/stats")]
async fn stats() -> Result<impl Responder> {
    log::debug!("webhook stats");

    let data = storage::serialized();

    Ok(actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK).json(&*data))
}
