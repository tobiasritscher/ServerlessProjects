mod routes;
mod stats_storage;

use actix_web::{App, HttpServer};
use futures::future::FutureExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(routes::get_scope()))
        .bind("127.0.0.1:8080")?
        .run();
    let runner = stats_storage::runner::runner();

    futures::select! {
        res = server.fuse() => res,
        _ = runner.fuse() => panic!("the runner should never be able to finish executing")
    }
}
