#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::*;
use self::schema::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct KvStore {
    conn: SqliteConnection,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            conn: establish_connection(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        use schema::map_values;

        let key_clone = key.clone();

        let new_value = MappedValue {
            key_value: key_clone,
            value: Some(value),
        };

        if (self.get(key.clone())).is_some() {
            self.remove(key.clone());
        }

        diesel::insert_into(map_values::table)
            .values(&new_value)
            .execute(&self.conn)
            .expect("Error saving new value.");
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        use self::schema::map_values::dsl::*;

        let results = map_values
            .filter(key_value.eq(key))
            .limit(1)
            .load::<MappedValue>(&self.conn)
            .expect("Error loading value.");

        for map_value in results {
            return map_value.value;
        }
        return None;
    }

    pub fn remove(&mut self, key: String) {
        use self::schema::map_values::dsl::*;

        let key_clone = key.clone();

        let num_deleted = diesel::delete(map_values.filter(key_value.eq(key)))
            .execute(&self.conn)
            .expect("Error deleting key.");
    }
}
