use super::schema::tasks;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub text: &'a str,
}

