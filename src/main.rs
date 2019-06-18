#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
use rocket::routes;

mod db;
mod schema;
mod person;

mod routes;
use routes::*;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/person", routes![create, update, delete])
        .mount("/persons", routes![read])
        .launch();
}
