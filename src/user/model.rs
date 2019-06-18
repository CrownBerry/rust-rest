use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};

use super::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    id: Option<i32>,
    username: String,
    password_hash: String,
}

impl User {
    pub fn read(id: Option<i32>, connection: &SqliteConnection) -> QueryResult<Vec<User>> {
        match id {
            Some(id) => users::table.find(id).load::<User>(connection),
            None => users::table.order(users::id.asc()).load::<User>(connection)
        }
    }
}
