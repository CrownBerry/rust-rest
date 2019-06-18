use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};

use super::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn read(id: Option<i32>, connection: &SqliteConnection) -> QueryResult<Vec<User>> {
        match id {
            Some(id) => users::table.find(id).load::<User>(connection),
            None => users::table.order(users::id.asc()).load::<User>(connection)
        }
    }

    pub fn create(user: User, connection: &SqliteConnection) -> QueryResult<User> {
        let pwhash = hash(user.password_hash, DEFAULT_COST)
            .expect("Failed to hash password");

        let user = User { id: None, username: user.username, password_hash: pwhash };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Failed to create new user");

        users::table.order(users::id.desc()).first(connection)
    }
}
