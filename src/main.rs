#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

extern crate tera;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("name".to_string(), "Tera".to_string());
    Template::render("index", &context)
}

#[get("/<name>")]
fn insert_template(name: &str) -> Template {
    println!("{}", name);
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("name".to_string(), name.to_string());
    Template::render("index", &context)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![insert_template])
        .attach(Template::fairing())
}
