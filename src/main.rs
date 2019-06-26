#![feature(proc_macro_hygiene, decl_macro)]

extern crate openssl;
#[macro_use]
extern crate diesel;

mod db;
mod person;
mod user;
mod api;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = person::mount(rocket);
    rocket = user::mount(rocket);
    rocket.launch();
}
