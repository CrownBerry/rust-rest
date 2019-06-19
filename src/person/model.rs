use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};

use super::schema::persons;
use diesel::result::Error;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
}

impl Person {
    pub fn create(person: Person, connection: &SqliteConnection) -> Result<Person, Error> {
        diesel::insert_into(persons::table)
            .values(&person)
            .execute(connection)?;

        persons::table
            .order(persons::id.desc())
            .first::<Person>(connection)
    }

    pub fn read(connection: &SqliteConnection) -> Result<Vec<Person>, Error> {
        persons::table
            .order(persons::id.asc())
            .load::<Person>(connection)
    }

    pub fn update(id: i32, person: Person, connection: &SqliteConnection) -> Result<usize, Error> {
        diesel::update(persons::table.find(id))
            .set(&person)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> Result<usize, Error>  {
        diesel::delete(persons::table.find(id))
            .execute(connection)
    }
}
