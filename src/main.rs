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

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;
mod db;

//#[get("/<name>/<age>")]
//fn hello(name: String, age: u8) -> String {
//    format!("Hello, {} year old named {}!", age, name)
//}

fn main() {
    dotenv().ok();

    //rocket::ignite()
    //    .mount("/hello", routes![hello])
    //    .launch();

    /*let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Miez de noapte"),
        author: String::from("Erin Hunter"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }*/
}
