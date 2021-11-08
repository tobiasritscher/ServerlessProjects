use rocket::{
    figment::{
        util::map,
        value::{Map, Value},
    },
    serde::{json, Deserialize, Serialize},
};

const DB_PATH_ENV: &str = "DB_PATH";

#[rocket_sync_db_pools::database("main_db")]
struct Database(rocket_sync_db_pools::rusqlite::Connection);

#[derive(Debug, Deserialize)]
struct SendBlog<'data> {
    timestamp: &'data str,
    text: &'data str,
}

#[derive(Debug, Serialize)]
struct SendBlockAnswer {
    id: usize,
}

#[rocket::post("/set", data = "<data>")]
fn set(data: json::Json<SendBlog<'_>>) -> json::Json<SendBlockAnswer> {
    todo!()
}

#[rocket::get("/get/<id>")]
fn get(id: usize) -> String {
    todo!()
}

#[rocket::launch]
fn rocket() -> _ {
    let db: Map<_, Value> = map! {
        "url" => std::env::var(DB_PATH_ENV).expect("missing DB_PATH_ENV").into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map! [ "main_db" => db ]));

    rocket::custom(figment).mount("/", rocket::routes![set, get])
}
