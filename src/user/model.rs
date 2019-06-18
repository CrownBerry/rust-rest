use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

use super::schema::users;

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
    pub fn read(id: Option<i32>, connection: &SqliteConnection) -> QueryResult<Vec<User>> {
        match id {
            Some(id) => users::table.find(id).load::<User>(connection),
            None => users::table.order(users::id.asc()).load::<User>(connection)
        }
    }

    pub fn create(user: User, connection: &SqliteConnection) -> QueryResult<User> {
        let pwhash = hash_password(user.password_hash);
        let user = User { id: None, username: user.username, password_hash: pwhash };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Failed to create new user");

        users::table.order(users::id.desc()).first(connection)
    }

    pub fn by_username_and_password(user_request: UserRequest, connection: &SqliteConnection) -> Option<User> {
        let res = users::table
            .filter(users::username.eq(user_request.username))
            .order(users::id)
            .first::<User>(connection);
        match res {
            Ok(user) => {
                match verify(user_request.password.to_string(), &user.password_hash.to_string()) {
                    Ok(_) => Some(user),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}

fn hash_password(password: String) -> String {
    hash(password, DEFAULT_COST)
        .expect("Failed to hash password")
}
