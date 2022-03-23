use super::schema::counter;

#[derive(Queryable)]
pub struct Counter {
    pub id: i32,
    pub value: i64,
}

#[derive(Insertable)]
#[table_name="counter"]
pub struct NewCounter {
    pub value: i64,
}

