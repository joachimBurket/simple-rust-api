#![feature(proc_macro_hygiene, decl_macro)] // tells rust we're using nightly features

use clokwerk::ScheduleHandle;
use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;

use crate::meteoswiss_api_client::MeteoSwissApiClient;

// import libs macros into our namespace
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate chrono_tz;

mod meteoswiss_api_client;
mod models;
mod schema;
mod views;

#[database("test_db")]
pub struct TestDbConn(diesel::SqliteConnection);

/// Configuring and starting a tasks scheduler
fn start_scheduler(api_client: MeteoSwissApiClient) -> ScheduleHandle {
    let mut scheduler = Scheduler::with_tz(chrono_tz::Europe::Zurich);

    // Getting Meteoswiss data each 10 minutes
    scheduler.every(10.minutes()).run(move || {
        println!("Periodic task is getting Meteoswiss data");
        let result = api_client.get_last_measures();
        match result {
            Ok(n) => println!(
                "Successfully recovered measures (print first 5): {:?}",
                &n[0..5]
            ),
            Err(e) => println!("Error: {}", e),
        }
    });

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    return thread_handle;
}

fn main() {
    let base_url = "https://data.geo.admin.ch".to_string();
    let stations_url = base_url.clone()
        + "/ch.meteoschweiz.messnetz-automatisch/ch.meteoschweiz.messnetz-automatisch_en.csv";
    let measures_url = base_url + "/ch.meteoschweiz.messwerte-aktuell/VQHA80.csv";
    let api_client = MeteoSwissApiClient::new(stations_url, measures_url);

    // Getting stations list
    let result = api_client.get_stations();
    match result {
        Ok(n) => println!(
            "Successfully recovered stations (print 5 first):\n {:?}",
            &n[0..5]
        ),
        Err(e) => println!("Error: {}", e),
    }

    // Configuring and starting scheduler
    let scheduler_thread_handle = start_scheduler(api_client);

    // Starting rocket 🚀
    rocket::ignite()
        .attach(TestDbConn::fairing())
        .mount("/", routes![views::index, views::list_users])
        .launch();

    // Cleanup
    scheduler_thread_handle.stop();
}
