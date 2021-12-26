use actix_web::{web, Responder, Result};

use crate::stats_storage::{Info, Stats};

pub fn get_scope() -> actix_web::Scope {
    web::scope("/").service(webhook).service(stats)
}

/// extract `Info` using serde
#[actix_web::post("/webhook")]
async fn webhook(_info: web::Json<Info>) -> Result<impl Responder> {
    // TODO:
    Ok("todo")
}

#[actix_web::get("/stats")]
async fn stats() -> Result<impl Responder> {
    let stats = Stats::new();

    Ok(web::Json(stats))
}
