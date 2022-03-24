#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::response::{Debug};

use self::diesel::prelude::*;

#[database("sqlite_database")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name="counter"]
struct Counter {
    id: i32,
    value: i64,
}

table! {
    counter (id) {
        id -> Integer,
        value -> BigInt,
    }
}

#[post("/")]
async fn increment(db: Db) -> Result<()> {
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|| {
            let val: u64 = counter::table.select(counter::value).get_result::<i64>(conn)? as u64;
            diesel::update(counter::table)
                .set(counter::value.eq((val + 1) as i64))
                .execute(&*conn)
        })
    }).await?;
    Ok(())
}

#[get("/")]
async fn get(db: Db) -> Result<String> {
    let value: u64 = db.run(move |conn| {
        counter::table.select(counter::value).get_result::<i64>(conn)
    }).await? as u64;
    Ok(value.to_string())
}


async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

pub fn diesel_sqlite_stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/", routes![increment, get])
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(diesel_sqlite_stage())
}
