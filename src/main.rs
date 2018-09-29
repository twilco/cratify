#![feature(plugin)]
#![plugin(rocket_codegen)]

use job_scheduler::{JobScheduler, Job};
use rocket::Request;
use slog::Drain;
// Required by slog::o! - do not remove unless slog::o! has also been removed.
use slog::kv;
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hello!"
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Route not found."
}

fn main() {
    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(build_logger());

    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();

    thread::spawn(|| {
        log::info!("Initiating subscription fulfillment scheduler.");
        let mut sched = build_subscription_scheduler();
        loop {
            sched.tick();
            thread::sleep(Duration::from_secs(60));
        }
    });

    rocket::ignite().catch(catchers![not_found]).mount("/", routes![index]).launch();
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
    log::info!("Attempting to retrieve or update crates.io index.");
    if crates_index::Index::new::<&str>("_index".into()).retrieve_or_update().is_err() {
        log::error!("Could not retrieve crates.io index.");
    }

    // loop through each subscription, and fulfill if necessary
}
