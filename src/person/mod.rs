use rocket::{delete, get, post, put};
use rocket::routes;
use rocket_contrib::json::Json;

use crate::api::{ApiResult, SuccessResponse};
use crate::api::auth::ApiKey;
use crate::db;

use self::model::Person;

pub mod model;
pub mod schema;

#[post("/", data = "<person>")]
fn create(_key: ApiKey, person: Json<Person>, connection: db::Connection) -> ApiResult<Person> {
    let insert = Person { id: None, ..person.into_inner() };

    let person = Person::create(insert, &connection)?;
    Ok(Json(person))
}

#[get("/")]
fn read(connection: db::Connection) -> ApiResult<Vec<Person>> {
    let persons = Person::read(&connection)?;
    Ok(Json(persons))
}

#[put("/<id>", data = "<person>")]
fn update(_key: ApiKey, id: i32, person: Json<Person>, connection: db::Connection) -> ApiResult<SuccessResponse> {
    let update = Person { id: Some(id), ..person.into_inner() };

    Person::update(id, update, &connection)?;
    Ok(Json(SuccessResponse::new(true)))
}

#[delete("/<id>")]
fn delete(_key: ApiKey, id: i32, connection: db::Connection) -> ApiResult<SuccessResponse> {
    Person::delete(id, &connection)?;
    Ok(Json(SuccessResponse::new(true)))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/person", routes![create, update, delete])
        .mount("/persons", routes![read])
}
