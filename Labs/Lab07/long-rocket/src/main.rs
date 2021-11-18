use rocket::{
    figment::{
        util::map,
        value::{Map, Value},
    },
    futures::{TryFutureExt, TryStreamExt},
    response::{status::Created, Debug},
    serde::{json::Json, Deserialize, Serialize},
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Connection, Database};

type Result<T, E = Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("main_db")]
struct Db(sqlx::SqlitePool);

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SendBlog {
    timestamp: String,
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct SendBlockAnswer {
    id: i64,
}

#[rocket::post("/set", data = "<data>")]
async fn create(
    mut db: Connection<Db>,
    data: Json<SendBlog>,
) -> Result<Created<Json<SendBlockAnswer>>> {
    let id = sqlx::query!(
        r#"
        INSERT INTO blogs
            (timestamp, text)
        VALUES
            (?, ?);
        "#,
        data.timestamp,
        data.text
    )
    .execute(&mut *db)
    .await?
    .last_insert_rowid();

    Ok(Created::new("/").body(Json(SendBlockAnswer { id })))
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Blog {
    id: i64,
    timestamp: String,
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Blogs {
    all: Vec<Blog>,
}

#[rocket::get("/get/<id>")]
async fn get(mut db: Connection<Db>, id: i64) -> Option<Json<Blog>> {
    let blog = sqlx::query!(
        r#"
        SELECT
            id, timestamp, text
        FROM blogs
        WHERE id = ?;
    "#,
        id
    )
    .fetch_one(&mut *db)
    .map_ok(|record| Blog {
        id: record.id,
        timestamp: record.timestamp,
        text: record.text,
    })
    .await
    .ok()?;

    Some(Json(blog))
}

#[rocket::get("/get")]
async fn get_all(mut db: Connection<Db>) -> Result<Json<Blogs>> {
    let blogs: Vec<_> = sqlx::query!(
        r#"
        SELECT
            id, timestamp, text
        FROM blogs;
        "#
    )
    .fetch(&mut *db)
    .map_ok(|record| Blog {
        id: record.id as _,
        timestamp: record.timestamp,
        text: record.text,
    })
    .try_collect()
    .await?;

    Ok(Json(Blogs { all: blogs }))
}

#[rocket::get("/")]
async fn index() -> &'static str {
    "Please add to the database via /set or get the list via /get/<id>"
}

async fn setup_db(rocket: Rocket<Build>) -> rocket::fairing::Result {
    // const TABLE: &str = r#"
    //     CREATE TABLE IF NOT EXISTS blogs (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         timestamp VARCHAR NOT NULL,
    //         text VARCHAR NOT NULL
    //     );"#;

    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                println!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[rocket::launch]
async fn rocket() -> _ {
    const DB_PATH_ENV: &str = "DB_PATH";

    let db: Map<_, Value> = map! {
        "url" => std::env::var(DB_PATH_ENV).expect("missing enviromental variable DB_PATH").into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map! [ "main_db" => db ]));

    rocket::custom(figment)
        .attach(Db::init())
        .attach(rocket::fairing::AdHoc::try_on_ignite(
            "Init database",
            setup_db,
        ))
        .mount("/", rocket::routes![create, get, get_all, index])
}
