#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod schema;
mod models;
mod db;
mod static_files;
mod routes;

use dotenv::dotenv;
use std::env;
use routes::*;
use static_files::*;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![show_all, new, show_by_id, update, delete, show_by_author],
        )
        .mount("/", routes![all, index])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
