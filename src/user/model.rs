use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

use super::schema::users;
use diesel::result::Error;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn read(id: Option<i32>, connection: &SqliteConnection) -> Result<Vec<User>, Error> {
        match id {
            Some(id) => users::table.find(id).load::<User>(connection),
            None => users::table.order(users::id.asc()).load::<User>(connection)
        }
    }

    pub fn create(user: User, connection: &SqliteConnection) -> Result<User, Error> {
        let user = User {
            id: None,
            username: user.username,
            password_hash: hash_password(user.password_hash)
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)?;

        users::table
            .order(users::id.desc())
            .first::<User>(connection)
    }

    pub fn find(user_request: UserRequest, connection: &SqliteConnection) -> Result<User, Error> {
        let user = users::table
            .filter(users::username.eq(user_request.username))
            .order(users::id)
            .first::<User>(connection)?;

        match verify(user_request.password.to_string(), &user.password_hash.to_string()) {
            Err(_) => Err(Error::NotFound),
            Ok(_) => Ok(user)
        }
    }
}

fn hash_password(password: String) -> String {
    hash(password, DEFAULT_COST)
        .expect("Failed to hash password")
}
