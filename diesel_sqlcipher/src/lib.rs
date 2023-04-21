pub mod models;
pub mod schema;

use diesel::{
    connection::SimpleConnection,
    prelude::*,
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn establish_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::establish("test.sqlite").unwrap();
    conn.batch_execute("PRAGMA key=password123").unwrap();
    conn.batch_execute("
        PRAGMA busy_timeout = 100;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA wal_autocheckpoint = 1000;
        PRAGMA wal_checkpoint(TRUNCATE);
        PRAGMA foreign_keys = ON;
    ").unwrap();

    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}
