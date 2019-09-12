#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use self::models::*;
use self::schema::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

struct KvStore {
    conn: SqliteConnection
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { conn: establish_connection() }
    }

    pub fn set(&mut self, key: String, value: String) {
        use schema::map_values;

        let new_value = MappedValue { key_value: key, value: Some(value) };

        diesel::insert_into(map_values::table)
            .values(&new_value)
            .execute(&self.conn)
            .expect(&format!("Error saving new value"))
    }
//
    pub fn get(&mut self, key: String) -> Option<String> {
        use self::schema::map_values::dsl::*;

        let results = map_values.load::<MappedValue>(&self.conn).expect("Error");
//        for result in results {
//            result.value;
//        }
        for mapped_value in results {
//            println!("Results: {}", mapped_value.key_value);
            return mapped_value.value;
        }
//        return Some("Test".parse().unwrap());
    }
//
//    pub fn remove(&mut self, key: String) {
//        panic!()
//    }
}

//mod schema {
//    table! {
//        map(key_value) {
//            key_value -> Text,
//            value -> Text,
//        }
//    }
//}

//mod schema {
//    table! {
//        map {
//            id -> Integer,
//            key -> Text,
//            value -> Text,
//        }
//    }
//}

//use schema::map;
//
//#[derive(Deserialize, Insertable)]
//#[table_name = "map"]
//pub struct MapForm<'a> {
//    key: &'a str,
//    value: &'a str,
//}
//
//#[derive(Queryable, PartialEq, Debug)]
//struct Map {
//    key: String,
//    value: String
//}


//

//pub struct KvStore {
//    conn:
//}

//pub struct KvStore {
//    map: HashMap<String, String>
//}
//
//impl KvStore {
//    pub fn new() -> KvStore {
//        KvStore { map: HashMap::new() }
//    }
//
//    pub fn set(&mut self, key: String, value: String) {
//        panic!()
//    }
//
//    pub fn get(&mut self, key: String) -> Option<String> {
//        panic!()
//    }
//
//    pub fn remove(&mut self, key: String) {
//        panic!()
//    }
//}