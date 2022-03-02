#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use models::NewCounter;
use schema::counter;

use diesel::prelude::*;
use diesel::connection::SimpleConnection;

use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    conn.batch_execute("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA wal_autocheckpoint = 1000;
        PRAGMA wal_checkpoint(TRUNCATE);
    ").unwrap();

    conn
}

fn main() {
   let conn = establish_connection();

    // Delete all rows.
    diesel::delete(counter::table).execute(&conn).unwrap();

    // Start the new counter at zero.
    diesel::insert_into(counter::table)
        .values(&NewCounter { value: 0 })
        .execute(&conn)
        .unwrap();

    use schema::counter::dsl::value;
    for _ in 0..10000 {
        diesel::update(counter::table)
            .set(value.eq(value + 1))
            .execute(&conn)
            .unwrap();
    }
}
