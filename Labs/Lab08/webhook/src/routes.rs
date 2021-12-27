use actix_web::dev::RequestHead;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Either, HttpResponse, HttpResponseBuilder, Responder, Result};

use crate::{model::Info, storage};

pub fn get_scope(path: &str) -> actix_web::Scope {
    web::scope(path).service(webhook).service(stats)
}

/// extract `Info` using serde
#[post("/webhook", guard = "webhook_guard")]
async fn webhook(info: Either<web::Json<Info>, String>) -> impl Responder {
    log::debug!("Received webhook data {:?}", info);

    // check type
    let info = match info {
        Either::Left(info) => info.0,
        Either::Right(raw_info) => match serde_json::from_str(&raw_info) {
            Ok(info) => info,
            Err(err) => {
                log::debug!("Failed to deserialize Json from payload. Request path: /webhook");
                log::debug!("JSON err {:?}", err);
                return HttpResponse::BadRequest().finish();
            }
        },
    };

    // TODO: send correctly serialized data to next function

    // move information direcly, so not to block the response for too long
    storage::store(info);

    // No response needed according to example
    let rep = HttpResponseBuilder::new(StatusCode::OK).finish();
    rep
}

fn webhook_guard(req: &RequestHead) -> bool {
    // allow for json and plain text content types
    use actix_web::http::header;
    match req.headers().get(header::CONTENT_TYPE) {
        Some(rtype) => match rtype.to_str() {
            Ok(ctype) => {
                ctype.starts_with(ContentType::json().essence_str())
                    || ctype.starts_with(ContentType::plaintext().essence_str())
            }
            Err(err) => {
                log::debug!("invalid content type used <{:?}>", err);
                false
            }
        },
        None => false,
    }
}

#[get("/stats")]
async fn stats() -> Result<impl Responder> {
    log::debug!("webhook stats");

    let data = storage::serialized();

    // Weird Content-Type needed according to example
    let ctype: mime::Mime = "application/json; charset=utf-8"
        .parse()
        .expect("Unable to parse content type...");

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(ContentType(ctype))
        .json(&*data))
}
