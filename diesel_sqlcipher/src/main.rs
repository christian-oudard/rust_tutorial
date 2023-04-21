use diesel::prelude::*;
use diesel_sqlcipher::establish_connection;
use diesel_sqlcipher::models::{Task, NewTask};

fn main() {
    let conn = &mut establish_connection();

    use diesel_sqlcipher::schema::tasks;

    // Create a task.
    let new_task = NewTask {
        text: "buy milk",
    };
    let task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .unwrap();

    dbg!(&task);

    // Mark it as completed.
    let task = diesel::update(&task)
        .set(tasks::completed.eq(true))
        .returning(Task::as_returning())
        .get_result(conn)
        .unwrap();

    dbg!(&task);

    // Delete all tasks.
    diesel::delete(tasks::table)
        .execute(conn)
        .unwrap();

    // Check the number of tasks.
    let count = tasks::table
        .count()
        .get_result::<i64>(conn)
        .unwrap();
    dbg!(&count);
}
