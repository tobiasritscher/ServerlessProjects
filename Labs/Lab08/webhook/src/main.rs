mod routes;
mod stats_storage;

use actix_web::{App, HttpServer};
use cfg_if::cfg_if;
use futures::future::FutureExt;

fn setup() {
    const LOGGER_ENV: &str = "RUST_LOG";
    cfg_if! {
        if #[cfg(debug_assertions)]  {
            const VALUE: &str = "debug";
        } else {
            const VALUE: &str = "warn";
        }
    }
    if std::env::var_os(LOGGER_ENV).is_none() {
        std::env::set_var(LOGGER_ENV, VALUE);
    }

    pretty_env_logger::init_timed();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup();
    let server = HttpServer::new(|| App::new().service(routes::get_scope()))
        .bind("127.0.0.1:8080")?
        .run();
    let runner = stats_storage::storage::handler();

    futures::select! {
        res = server.fuse() => res,
        _ = runner.fuse() => panic!("the runner should never be able to finish executing")
    }
}
