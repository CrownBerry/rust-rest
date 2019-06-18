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
pub fn login(credentials: Json<UserRequest>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
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
pub fn read(_key: ApiKey, id: Option<i32>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    match User::read(id, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/", data = "<user>")]
pub fn create(user: Json<UserRequest>, connection: db::Connection) -> Result<Json<JsonValue>, Status> {
    let insert = User { id: None, username: user.username.to_string(), password_hash: user.password.to_string() };
    match User::create(insert, &connection) {
        Ok(res) => Ok(Json(json!(res))),
        Err(_) => Err(Status::BadRequest)
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/", routes![read])
        .mount("/user", routes![create])
        .mount("/login", routes![login])
}
