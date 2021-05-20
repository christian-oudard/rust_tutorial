#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::{connection::SimpleConnection, prelude::*};

use dotenv::dotenv;
use std::env;

use self::models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    println!("test");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let encryption_key = "pw";

    // Check whether the database is already encrypted.
    match conn.batch_execute("SELECT count(*) from sqlite_master;") {
        Err(_) => (),
        Ok(_) => {
            conn.batch_execute(&format!(
                "
                ATTACH DATABASE 'encrypted.db' AS encrypted KEY '{}';
                SELECT sqlcipher_export('encrypted');
                DETACH DATABASE encrypted;
                ",
                encryption_key
            ))
            .expect("could not convert database");

            // STUB, replace unencrypted database with the new encrypted one.
        }
    }

    conn.batch_execute(&format!("PRAGMA key=\"{}\";", encryption_key))
        .expect("Could not send encryption key.");
    conn.batch_execute("SELECT count(*) from sqlite_master")
        .expect("Could not decrypt database");

    conn
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}
