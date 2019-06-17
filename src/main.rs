#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::{Json, JsonValue};

mod db;
mod schema;
mod person;
use person::Person;

#[post("/", data = "<person>")]
fn create(person: Json<Person>, connection: db::Connection) -> Json<Person> {
    let insert = Person {id: None, ..person.into_inner() };
    Json(Person::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(Person::read(&connection)))
}

#[put("/<id>", data = "<person>")]
fn update(id: i32, person: Json<Person>, connection: db::Connection) -> Json<JsonValue> {
    let update = Person {id: Some(id), ..person.into_inner() };
    Json(json!({
        "success": Person::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Person::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
