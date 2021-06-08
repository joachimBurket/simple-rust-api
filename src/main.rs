#![feature(proc_macro_hygiene, decl_macro)] // tells rust we're using nightly features

use clokwerk::ScheduleHandle;
use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;

use crate::meteoswiss_api_client::MeteoSwissApiClient;


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

/// Configuring and starting a tasks scheduler
fn start_scheduler() -> ScheduleHandle {
    let mut scheduler = Scheduler::with_tz(chrono_tz::Europe::Zurich);
    
    // Getting Meteoswiss data each 10 minutes
    scheduler.every(10.minutes()).run(|| {
        println!("Periodic task is getting Meteoswiss data");
        let api_client = MeteoSwissApiClient::new(
            String::from("https://data.geo.admin.ch/ch.meteoschweiz.messwerte-aktuell/VQHA80.csv")
        );
        api_client.get_last_measures();
    });

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    return thread_handle;
}

fn main() {
    // Configuring and starting scheduler
    let scheduler_thread_handle = start_scheduler();

    // Starting rocket 🚀
    rocket::ignite()
        .attach(TestDbConn::fairing())
        .mount("/", routes![views::index, views::list_users])
        .launch();
    
    // Cleanup
    scheduler_thread_handle.stop();
}