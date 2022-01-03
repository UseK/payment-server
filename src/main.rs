#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rusqlite::params;

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
    let manager = SqliteConnectionManager::file("payment.db");
    let pool = Pool::new(manager).expect("Fail to init pool");
    let conn = pool.get().expect("Fail to get connection from pool");
    conn.execute(
        "create table if not exists payment (
            date text not null,
            name text not null,
            value integer
        )",
        params![],
    )
    .expect("Failed to create table");

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![insert_template])
        .attach(Template::fairing())
}
