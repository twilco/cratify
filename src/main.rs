#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate slog;

#[macro_use]
extern crate slog_scope;

pub(crate) mod app_env;
pub(crate) mod db;

use self::app_env::AppEnv;
use actix_web::{
    fs::NamedFile, fs::StaticFiles, http::Method, server, App, HttpRequest, Responder, State,
};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use job_scheduler::{Job, JobScheduler};
use r2d2::Pool;
use slog::Drain;
use std::fs::OpenOptions;
use std::thread;
use std::time::Duration;

// Embed migrations into the binary rather than requiring them to be run through some outside process.
embed_migrations!("migrations");

fn main() -> Result<(), Box<std::error::Error>> {
    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(build_logger());
    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();
    slog_scope::scope(&slog_scope::logger(), cratify);

    Ok(())
}

fn cratify() {
    // Use dotenv to load environment variables in to the system environment, so std::env can use
    // them elsewhere in the application.  Only necessary when running locally outside of Docker -
    // we use Docker Compose to load in the proper .env file in that situation
    if let Err(err) = dotenv::from_filename("local.env") {
        info!(
            "error trying to load local.env.  this is probably not a problem if running in docker. err was: {}",
            err
        );
    }

    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(
        std::env::var("CRATIFY_DATABASE_URL")
            .expect("CRATIFY_DATABASE_URL env variable is required"),
    );
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("could not open database pool");

    info!("checking for unrun migrations");
    let mut migration_out = Vec::new();
    embedded_migrations::run_with_output(
        &pool
            .get()
            .expect("could not get database conn from pool when running migrations"),
        &mut migration_out,
    )
    .expect("error running migrations");

    if migration_out.len() > 0 {
        info!(
            "migrations run: \n{}",
            String::from_utf8_lossy(&migration_out)
        );
    } else {
        info!("no unran migrations found");
    }

    thread::spawn(|| {
        info!("initiating subscription fulfillment scheduler.");
        let mut sched = build_subscription_scheduler();
        loop {
            sched.tick();
            thread::sleep(Duration::from_secs(60));
        }
    });

    match std::env::var("CRATIFY_APP_ENV") {
        Ok(env_str) => match env_str.as_str() {
            "local" => {
                server::new(move || build_app(pool.clone(), AppEnv::Local))
                    .bind("0.0.0.0:8080")
                    .expect("couldn't start actix web server")
                    .run();
            }
            "prod" => {
                server::new(move || build_app(pool.clone(), AppEnv::Prod))
                    .bind("0.0.0.0:80")
                    .expect("couldn't start actix web server")
                    .run();
            }
            _ => panic!("unexpected environment found: {}", env_str),
        },
        Err(e) => panic!("unable to find CRATIFY_APP_ENV - err was: {}", e),
    }
}

fn build_app(conn_pool: Pool<ConnectionManager<PgConnection>>, env: AppEnv) -> App<AppState> {
    let static_handler = match env {
        AppEnv::Local => StaticFiles::new("./frontend/build/static").unwrap(),
        AppEnv::Prod => StaticFiles::new("./frontend/build/static").unwrap(),
    };

    App::with_state(AppState { conn_pool, env })
        .resource("/api", |res| {
            res.method(Method::GET)
                .f(|_r: &HttpRequest<AppState>| "api")
        })
        .resource("/static/{tail:.*}", |res| {
            res.method(Method::GET).h(static_handler)
        })
        .resource("/{tail:.*}", |res| res.method(Method::GET).with(frontend))
        .default_resource(|res| res.f(default_route))
}

struct AppState {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
    env: AppEnv,
}

fn frontend(state: State<AppState>) -> impl Responder {
    match state.env {
        AppEnv::Local => NamedFile::open("frontend/build/index.html"),
        AppEnv::Prod => NamedFile::open("frontend/build/index.html"),
    }
}

fn default_route(_req: &HttpRequest<AppState>) -> impl Responder {
    "route not found"
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
    info!("attempting to retrieve or update crates.io index");
    if crates_index::Index::new::<&str>("_index".into())
        .retrieve_or_update()
        .is_err()
    {
        error!("could not retrieve crates.io index");
    }

    // loop through each subscription, and fulfill if necessary
}
