use rocket::{get, http::Status};
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;

use self::model::User;

pub mod model;
pub mod schema;

#[get("/users?<id>")]
pub fn read(id: Option<i32>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    match User::read(id, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/", routes![read])
}
