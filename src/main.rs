#![feature(proc_macro_hygiene, decl_macro)] // tells rust we're using nightly features

// import libs macros into our namespace
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;
extern crate chrono_tz;

mod models;
mod schema;
mod views;
mod meteoswiss_api_client;

#[database("test_db")]
pub struct TestDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(TestDbConn::fairing())
        .mount("/", routes![views::index, views::list_users])
        .launch();
}