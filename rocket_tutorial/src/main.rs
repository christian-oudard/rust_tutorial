#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{
    config::{Config, Environment},
    request::{self, FromRequest, Request},
    State,
};
use rocket::outcome::Outcome::{self, *};

struct TestState {
    password: String,
}

struct TestGuard;

impl<'a, 'r> FromRequest<'a, 'r> for TestGuard {
    type Error = ();

    fn from_request(
        request: &'a Request<'r>,
    ) -> Outcome<Self, (rocket::http::Status, Self::Error), ()> {
        let state = request.guard::<State<TestState>>()?;
        Success(TestGuard)
    }
}

#[get("/")]
fn index(_guard: TestGuard, _state: State<TestState>) -> String {
    "Hello, world!\n".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(TestState {
            password: "pw".to_string(),
        })
        .launch();
}
