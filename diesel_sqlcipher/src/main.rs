use diesel::{
    prelude::*,
    SqliteConnection,
    sql_query,
};

#[allow(unused_imports)] // Needed for embedded_migrations!
#[macro_use]

extern crate diesel_migrations;
embed_migrations!("migrations/");

fn create_connection() -> SqliteConnection {
  let database_url = "test_encrypted.db";
  SqliteConnection::establish(&database_url)
      .expect("Error connecting to database.")
}

fn main() {
    let conn = create_connection();
    sql_query("PRAGMA key=\"encrypted\";").execute(&conn)
        .expect("Could not send encryption key.");
    embedded_migrations::run(&conn)
        .expect("Failed to run migrations.");
}
