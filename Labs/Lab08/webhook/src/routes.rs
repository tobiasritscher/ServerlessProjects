use actix_web::dev::RequestHead;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Either, HttpResponse, HttpResponseBuilder, Responder};

use crate::{model::Info, storage};

pub fn get_scope(path: &str) -> actix_web::Scope {
    web::scope(path).service(webhook).service(stats)
}

/// extract `Info` using serde
#[post("/webhook", guard = "webhook_guard")]
async fn webhook(
    storage: web::Data<storage::Storage<Info>>,
    info: Either<web::Json<Info>, String>,
) -> impl Responder {
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

    // move information direcly, so not to block the response for too long
    // TODO: FIX THIS
    storage.store(info).await;

    // No response needed according to example
    HttpResponseBuilder::new(StatusCode::OK).finish()
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
async fn stats(storage: web::Data<storage::Storage<Info>>) -> impl Responder {
    log::debug!("webhook stats");

    let data = storage.data().await;

    log::debug!("sending <{:?}>", data);

    // // Weird Content-Type needed according to example
    let ctype: mime::Mime = "application/json; charset=utf-8"
        .parse()
        .expect("Unable to parse content type...");

    HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(ContentType(ctype))
        .json(data)
}
