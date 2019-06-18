use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde_derive::{Deserialize, Serialize};

use super::schema::persons;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
}

impl Person {
    pub fn create(person: Person, connection: &SqliteConnection) -> Person {
        diesel::insert_into(persons::table)
            .values(&person)
            .execute(connection)
            .expect("Error creating new person");

        persons::table
            .order(persons::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read(connection: &SqliteConnection) -> Vec<Person> {
        persons::table
            .order(persons::id.asc())
            .load::<Person>(connection)
            .unwrap()
    }

    pub fn update(id: i32, person: Person, connection: &SqliteConnection) -> bool {
        diesel::update(persons::table.find(id))
            .set(&person)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(persons::table.find(id))
            .execute(connection)
            .is_ok()
    }
}

