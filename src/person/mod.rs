use rocket::{delete, get, post, put};
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;
use crate::user::auth::ApiKey;

use self::model::Person;

pub mod model;
pub mod schema;

#[post("/", data = "<person>")]
fn create(_key: ApiKey, person: Json<Person>, connection: db::Connection) -> Json<Person> {
    let insert = Person { id: None, ..person.into_inner() };
    Json(Person::create(insert, &connection))
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

#[delete("/<id>")]
fn delete(_key: ApiKey, id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Person::delete(id, &connection)
    }))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/person", routes![create, update, delete])
        .mount("/persons", routes![read])
}
