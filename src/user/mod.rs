use rocket::{post, get, http::Status};
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;

use self::model::User;
use crate::user::model::UserRequest;

pub mod model;
pub mod schema;

#[get("/users?<id>")]
pub fn read(id: Option<i32>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    match User::read(id, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/", data = "<user>")]
pub fn create(user: Json<UserRequest>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {

    let insert = User {id: None, username: user.username.to_string(), password_hash: user.password.to_string()};
    match User::create(insert, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::BadRequest)
    }
}2

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/", routes![read])
        .mount("/user", routes![create])
}
