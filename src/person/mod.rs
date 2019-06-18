use rocket::{delete, get, post, put, http::Status};
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;

use self::model::Person;
use crate::user::auth::ApiKey;

pub mod model;
pub mod schema;

#[post("/", data = "<person>")]
fn create(_key: ApiKey, person: Json<Person>, connection: db::Connection) -> Json<Person> {
    let insert = Person { id: None, ..person.into_inner() };
    Json(Person::create(insert, &connection))
}
#[post("/", rank = 2)]
fn create_error() -> Status {
    Status::Unauthorized
}

#[get("/")]
fn read(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(Person::read(&connection)))
}

#[put("/<id>", data = "<person>")]
fn update(_key: ApiKey, id: i32, person: Json<Person>, connection: db::Connection) -> Json<JsonValue> {
    let update = Person { id: Some(id), ..person.into_inner() };
    Json(json!({
        "success": Person::update(id, update, &connection)
    }))
}
#[put("/<id>", rank = 2)]
fn update_error(id: i32) -> Status {
    Status::Unauthorized
}

#[delete("/<id>")]
fn delete(_key: ApiKey, id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Person::delete(id, &connection)
    }))
}
#[delete("/<id>", rank = 2)]
fn delete_error(id: i32) -> Status {
    Status::Unauthorized
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/person", routes![create, create_error, update, update_error, delete, delete_error])
        .mount("/persons", routes![read])
}
