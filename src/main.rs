#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;

use job_scheduler::{Job, JobScheduler};
use rocket::Request;
use slog::Drain;
// Required by slog::o! - do not remove unless slog::o! has also been removed.
use slog::kv;
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;

mod schema;
mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello!"
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Route not found."
}

// Embed migrations into the binary rather than requiring them to be run through some outside process.
embed_migrations!("migrations");

fn main() -> Result<(), Box<std::error::Error>> {
    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(build_logger());
    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();

    // Use dotenv to load environment variables in to the system environment, so std::env can use
    // them elsewhere in the application.  Only necessary when running locally outside of Docker -
    // we use Docker Compose to load in the proper .env file in that situation
    if let Err(err) = dotenv::from_filename("local.env") {
        log::info!(
            "error trying to load local.env.  this is probably not a problem if running in docker. err was: {}",
            err
        );
    }

    let manager: diesel::r2d2::ConnectionManager<PgConnection> =
        diesel::r2d2::ConnectionManager::new(
            std::env::var("CRATIFY_DATABASE_URL")
                .expect("CRATIFY_DATABASE_URL env variable is required"),
        );
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("could not open database pool");

    log::info!("checking for unrun migrations");
    let mut migration_out = Vec::new();
    embedded_migrations::run_with_output(&pool.get()?, &mut migration_out)
        .expect("error running migrations");
    if migration_out.len() > 0 {
        log::info!(
            "migrations run: \n{}",
            String::from_utf8_lossy(&migration_out)
        );
    } else {
        log::info!("no unran migrations found");
    }

    thread::spawn(|| {
        log::info!("Initiating subscription fulfillment scheduler.");
        let mut sched = build_subscription_scheduler();
        loop {
            sched.tick();
            thread::sleep(Duration::from_secs(60));
        }
    });

    rocket::ignite()
        .catch(catchers![not_found])
        .mount("/", routes![index])
        .launch();

    Ok(())
}

fn build_logger() -> slog::Logger {
    let log_path = "logs.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let file_dec = slog_term::PlainSyncDecorator::new(file);
    let file_drain = slog_term::FullFormat::new(file_dec).build().fuse();

    let term_dec = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let term_drain = slog_term::FullFormat::new(term_dec).build().fuse();

    let dupe_drain = slog::Duplicate::new(file_drain, term_drain).fuse();
    slog::Logger::root(dupe_drain, slog::o!())
}

fn build_subscription_scheduler() -> JobScheduler<'static> {
    let mut sched = JobScheduler::new();

    // Run every three hours
    let schedule = "0 0 0,3,6,9,12,15,18,21 * * * *";
    sched.add(Job::new(schedule.parse().unwrap(), fulfill_subscriptions));

    sched
}

fn fulfill_subscriptions() {
    log::info!("attempting to retrieve or update crates.io index");
    if crates_index::Index::new::<&str>("_index".into())
        .retrieve_or_update()
        .is_err()
    {
        log::error!("could not retrieve crates.io index");
    }

    // loop through each subscription, and fulfill if necessary
}
