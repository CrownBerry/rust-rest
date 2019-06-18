use rocket::{get, http::Status, post};
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use crate::db;

use self::auth::ApiKey;
use self::model::{User, UserRequest};

pub mod model;
pub mod schema;
pub mod auth;

#[post("/", data = "<credentials>")]
fn login(credentials: Json<UserRequest>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    match User::by_username_and_password(credentials.into_inner(), &connection) {
        None => {
            Err(Status::BadRequest)
        }
        Some(user) => {
            let token = auth::generate_token(user).unwrap();
            Ok(Json(json!({"success": true, "token": token})))
        }
    }
}

#[get("/users?<id>")]
fn read(_key: ApiKey, id: Option<i32>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    match User::read(id, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::NotFound)
    }
}
#[get("/users?<id>", rank = 2)]
fn read_error(id: Option<i32>) -> Result<Json<JsonValue>, Status> {
    Err(Status::Unauthorized)
}

#[post("/", data = "<user>")]
fn create(_key: ApiKey, user: Json<UserRequest>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    let insert = User { id: None, username: user.username.to_string(), password_hash: user.password.to_string() };
    match User::create(insert, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::BadRequest)
    }
}
#[post("/", rank = 2)]
fn create_error() -> Result<Json<JsonValue>, Status> {
    Err(Status::Unauthorized)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/", routes![read, read_error])
        .mount("/user", routes![create, create_error])
        .mount("/login", routes![login])
}
