use rocket::{get, post};
use rocket::routes;
use rocket_contrib::json::Json;

use crate::api::{auth, ApiResult};
use crate::api::auth::{ApiKey, TokenResponse};
use crate::db;

use self::model::{User, UserRequest};

pub mod model;
pub mod schema;

#[post("/", data = "<credentials>")]
fn login(credentials: Json<UserRequest>, connection: db::Connection) -> ApiResult<TokenResponse> {
    let user = User::find(credentials.into_inner(), &connection)?;
    let token = auth::generate_token(user).unwrap();
    Ok(Json(TokenResponse::from(token)))
}

#[get("/users?<id>")]
fn read(_key: ApiKey, id: Option<i32>, connection: db::Connection) -> ApiResult<Vec<User>> {
    let users = User::read(id, &connection)?;
    Ok(Json(users))
}

#[post("/", data = "<user>")]
fn create(_key: ApiKey, user: Json<UserRequest>, connection: db::Connection) -> ApiResult<User> {
    let insert = User {
        id: None,
        username: user.username.to_string(),
        password_hash: user.password.to_string()
    };

    let user = User::create(insert, &connection)?;
    Ok(Json(user))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/", routes![read])
        .mount("/user", routes![create])
        .mount("/login", routes![login])
}
