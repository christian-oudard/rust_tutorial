#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use models::NewCounter;

use diesel::prelude::*;
use diesel::connection::SimpleConnection;
use diesel::result::Error;

const DATABASE_URL: &str = "test.db";

fn establish_connection() -> SqliteConnection {
    let conn = SqliteConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

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

    use schema::counter::dsl::{counter, value};

    // Delete all rows.
    diesel::delete(counter).execute(&conn).unwrap();

    // Start the new counter at zero.
    diesel::insert_into(counter)
        .values(&NewCounter { value: 0 })
        .execute(&conn)
        .unwrap();

    for _ in 0..255 {
        conn.transaction::<_, Error, _>(|| {
            let val = counter.select(value).get_result::<i64>(&conn).unwrap();
            let mut val = val as u64;
            val += 1<<56;
            println!("{}", val);

            diesel::update(counter)
                .set(value.eq(val as i64))
                .execute(&conn)
        }).unwrap();
    }
    println!("{}", u64::MAX);
}
