#![feature(proc_macro_hygiene, decl_macro)]

use std::time::Duration;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate rocket_contrib;
use diesel::connection::{Connection, SimpleConnection};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::databases::Poolable;

pub mod models;
pub mod schema;
use models::NewCounter;
use schema::counter::dsl::{counter, value};

struct DbConn(pub r2d2::PooledConnection<<SqliteConnection as Poolable>::Manager>);
struct DbConnPool(r2d2::Pool<<SqliteConnection as Poolable>::Manager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        use ::rocket::{http::Status, Outcome};
        let pool = request.guard::<::rocket::State<DbConnPool>>()?;

        match pool.0.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl ::std::ops::Deref for DbConn {
    type Target = SqliteConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct Customizer;

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for Customizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        Ok((|| {
            // Set busy timeout first, to minimize database locking.
            conn.batch_execute("PRAGMA busy_timeout = 1000;")?;
            conn.batch_execute("
                PRAGMA journal_mode = WAL;          -- better write-concurrency
                PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
                PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
                PRAGMA wal_checkpoint(TRUNCATE);    -- free some space by truncating possibly massive WAL files from the last run.
                PRAGMA foreign_keys = ON;
            ")?;
            Ok(())
        })().map_err(diesel::r2d2::Error::QueryError)?)
    }
}

#[get("/")]
fn index(conn: DbConn) -> String {
    let val: u64 = counter.select(value).get_result::<i64>(&*conn).unwrap() as u64;
    format!("{}", val).to_string()
}

#[post("/")]
fn index_post(conn: DbConn) {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let val: u64 = counter.select(value).get_result::<i64>(&*conn).unwrap() as u64;
        diesel::update(counter)
            .set(value.eq((val + 1) as i64))
            .execute(&*conn).unwrap();
        Ok(())
    }).unwrap();
}

fn main() {
    let manager = ConnectionManager::new("db.sqlite");
    let pool = Pool::builder()
        .connection_customizer(Box::new(Customizer))
        .max_size(5)
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .expect("Could not create database connection pool.");

    // Delete all rows and start the new counter at zero.
    let conn = pool.get().unwrap();
    diesel::delete(counter).execute(&conn).unwrap();
    diesel::insert_into(counter)
        .values(&NewCounter { value: 0 })
        .execute(&conn)
        .unwrap();

    rocket::ignite()
        .manage(DbConnPool(pool))
        .mount("/", routes![index, index_post])
        .launch();
}
