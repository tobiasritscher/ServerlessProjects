use rocket::{
    figment::{
        util::map,
        value::{Map, Value},
    },
    response::{status::Created, Debug},
    serde::{json::Json, Deserialize, Serialize},
    Build, Rocket,
};

use rocket_sync_db_pools::rusqlite;

type Result<T, E = Debug<rusqlite::Error>> = std::result::Result<T, E>;

#[rocket_sync_db_pools::database("main_db")]
struct InternalDB(rusqlite::Connection);

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SendBlog {
    timestamp: String,
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct SendBlockAnswer {
    id: usize,
}

#[rocket::post("/set", data = "<data>")]
async fn create(db: InternalDB, data: Json<SendBlog>) -> Result<Created<Json<SendBlockAnswer>>> {
    const INSERT: &str = r#"
        INSERT INTO blogs
            (timestamp, text)
        VALUES
            (?1, ?2);
    "#;

    const LAST_ID: &str = r#"
        SELECT last_insert_rowid();
    "#;

    db.run(move |conn| conn.execute(INSERT, rusqlite::params![data.timestamp, data.text]))
        .await?;

    let id = db
        .run(|conn| conn.query_row(LAST_ID, rusqlite::params![], |v| Ok(v.get(0)?)))
        .await?;

    Ok(Created::new("/").body(Json(SendBlockAnswer { id })))
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Blog {
    id: usize,
    timestamp: String,
    text: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Blogs {
    all: Vec<Blog>,
}

#[rocket::get("/get/<id>")]
async fn get(db: InternalDB, id: usize) -> Option<Json<Blog>> {
    const SELECT: &str = r#"
        SELECT
            id, timestamp, text
        FROM blogs
        WHERE id = ?1;
    "#;
    let blog = db
        .run(move |conn| {
            conn.query_row(SELECT, rusqlite::params![id], |v| {
                Ok(Blog {
                    id: v.get(0)?,
                    timestamp: v.get(1)?,
                    text: v.get(2)?,
                })
            })
        })
        .await
        .ok()?;

    Some(Json(blog))
}

#[rocket::get("/get")]
async fn get_all(db: InternalDB) -> Result<Json<Blogs>> {
    const SELECT: &str = r#"
        SELECT
            id, timestamp, text
        FROM blogs;
    "#;
    let blogs = db
        .run(|conn| {
            conn.prepare(SELECT)?
                .query_map(rusqlite::params![], |row| {
                    Ok(Blog {
                        id: row.get(0)?,
                        timestamp: row.get(1)?,
                        text: row.get(2)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()
        })
        .await?;

    Ok(Json(Blogs { all: blogs }))
}

#[rocket::get("/")]
async fn index() -> &'static str {
    "Please add to the database via /set or get the list via /get/<id>"
}

async fn setup_db(rocket: Rocket<Build>) -> Rocket<Build> {
    const TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS blogs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp VARCHAR NOT NULL,
            text VARCHAR NOT NULL
        );"#;

    InternalDB::get_one(&rocket)
        .await
        .expect("database mounted")
        .run(|con| con.execute(TABLE, rusqlite::params![]))
        .await
        .expect("can run init of the database");

    rocket
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
        .attach(InternalDB::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite("Init database", setup_db))
        .mount("/", rocket::routes![create, get, get_all, index])
}
