mod model;
mod routes;
mod storage;
mod timestamp;

use actix_web::{App, HttpServer};
use cfg_if::cfg_if;
use futures::future::FutureExt;

fn setup() -> String {
    use std::env::{set_var, var};
    const LOGGER_ENV: &str = "RUST_LOG";
    cfg_if! {
        if #[cfg(debug_assertions)]  {
            const VALUE: &str = "debug";
        } else {
            const VALUE: &str = "warn";
        }
    }
    if var(LOGGER_ENV).is_err() {
        set_var(LOGGER_ENV, VALUE);
    }

    pretty_env_logger::init_timed();

    const BASE_ADDRESS: &str = "127.0.0.1:8000";
    const SERVER_ADDRESS: &str = "SERVER_ADDRESS";
    const SERVER_PORT: &str = "SERVER_PORT";

    let server_addr = var(SERVER_ADDRESS);
    let server_port = var(SERVER_PORT);

    match (server_addr, server_port) {
        (Ok(addr), Ok(port)) => format!("{}:{}", addr, port),
        _ => BASE_ADDRESS.to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_addr = setup();

    log::info!("setting up server on {}", server_addr);

    // initialize the http server
    let server = HttpServer::new(|| App::new().service(routes::get_scope("")))
        .bind(server_addr)?
        .run();

    // initialize the temporary data storage
    let runner = storage::handler();

    futures::select! {
        res = server.fuse() => res,
        _ = runner.fuse() => panic!("the runner should never be able to finish executing")
    }
}
