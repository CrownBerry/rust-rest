use rocket::{delete, get, post, put};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;

use super::person::Person;

#[post("/", data = "<person>")]
pub fn create(person: Json<Person>, connection: db::Connection) -> Json<Person> {
    let insert = Person { id: None, ..person.into_inner() };
    Json(Person::create(insert, &connection))
}

#[get("/")]
pub fn read(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(Person::read(&connection)))
}

#[put("/<id>", data = "<person>")]
pub fn update(id: i32, person: Json<Person>, connection: db::Connection) -> Json<JsonValue> {
    let update = Person { id: Some(id), ..person.into_inner() };
    Json(json!({
        "success": Person::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Person::delete(id, &connection)
    }))
}
