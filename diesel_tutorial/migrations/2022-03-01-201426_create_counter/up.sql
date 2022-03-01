CREATE TABLE counter (
#[macro_use]
extern crate diesel;
    id INTEGER NOT NULL PRIMARY KEY,
    value INTEGER NOT NULL
);
