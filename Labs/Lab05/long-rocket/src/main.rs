use rocket::FromForm;
use rocket_dyn_templates::{Engines, Template};

#[rocket::get("/db")]
fn db() -> String {
    format!("tmp {}", "alive?")
}

#[derive(rocket::FromForm)]
struct Task<'d> {
    text: &'d str,
}

#[rocket::post("/form", data = "<task>")]
fn form_submitted(task: rocket::form::Form<Task<'_>>) -> Template {}

fn form() -> Template {
    todo!()
}

#[rocket::get("/")]
fn root() -> String {
    format!("Welcome, {}", "b")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::try_custom(setup))
        .mount("/", rocket::routes![root, db])
}

fn setup(engine: &mut Engines) -> Result<(), Box<dyn std::error::Error>> {
    macro_rules! t {
        ($v:literal) => {
            include_str!(concat!("../templates/", $v))
        };
    }
    let err = engine.tera.add_raw_templates([
        ("base.html", t!("base.html")),
        ("form.html", t!("form.html")),
        ("macro.html", t!("macro.html.tera")),
        ("form_submitted.html", t!("form_submitted.html")),
    ]);
    err.map_err(|err| Box::new(err) as _)
}
