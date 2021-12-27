mod db;
mod model;
mod routes;
mod storage;
mod timestamp;

use actix_web::HttpServer;
use cfg_if::cfg_if;
use futures::FutureExt;

fn setup() -> (String, Option<String>) {
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

    const BASE_ADDRESS_ENV: &str = "127.0.0.1:8000";
    const SERVER_ADDRESS_ENV: &str = "ADDRESS";
    const SERVER_PORT_ENV: &str = "PORT";
    const DB_ADDRESS_ENV: &str = "DB";

    let server_addr = var(SERVER_ADDRESS_ENV);
    let server_port = var(SERVER_PORT_ENV);
    let db_addr = var(DB_ADDRESS_ENV);

    let server = match (server_addr, server_port) {
        (Ok(addr), Ok(port)) => format!("{}:{}", addr, port),
        _ => BASE_ADDRESS_ENV.to_string(),
    };

    if db_addr.is_err() {
        log::warn!(
            "No <{}> environment was variable found, so there will be no webhook forwarding.",
            DB_ADDRESS_ENV
        );
    }

    (server, db_addr.ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (server_addr, db_addr) = setup();

    log::info!("setting up server on {}", server_addr);

    // initialize the temporary data storage and give the db handler to it to be called
    let mut storage_handler = storage::StorageHandler::new();

    // needs to be done now as runner will borrow storage_handler muatable
    let storage = storage_handler.get_storage();

    let runner = storage_handler.handler(|info| async {
        db::handle(info, db_addr.as_deref()).await;
    });

    // initialize the http server
    let server = HttpServer::new(move || {
        use actix_web::{web, App};
        App::new()
            .app_data(web::Data::new(storage.clone()))
            .service(routes::get_scope(""))
    })
    .bind(server_addr)?
    .run();

    futures::select! {
        res = server.fuse() => res,
        _ = runner.fuse() => panic!("the runner should never be able to finish executing")
    }
}
