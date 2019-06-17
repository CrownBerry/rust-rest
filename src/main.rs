#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

mod person;
use person::Person;

#[post("/", data = "<person>")]
fn create(person: Json<Person>) -> Json<Person> {
    person
}

#[get("/")]
fn read() -> Json<JsonValue> {
    Json(json!([
        "person 1", 
        "person 2"
    ]))
}

#[put("/<id>", data = "<person>")]
fn update(id: i32, person: Json<Person>) -> Json<Person> {
    person
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}


fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
