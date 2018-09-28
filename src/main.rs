#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::Request;
use slog::Drain;
use slog::kv;
use slog::o;
use std::fs::OpenOptions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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

    log::info!("Attempting to retrieve or update crates.io index.");
    if crates_index::Index::new::<&str>("_index".into()).retrieve_or_update().is_err() {
        log::error!("Could not retrieve crates.io index.");
    }

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
    slog::Logger::root(dupe_drain, o!())
}
