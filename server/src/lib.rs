use dotenv::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::http::uri::fmt::Kind::Path;
use rocket::{catch, catchers, launch, routes};
use std::path;
use log::LevelFilter;

extern crate rocket;
use rocket::serde::json::json;
use rocket_cors::{Cors, CorsOptions};
use serde_json::Value;
use crate::fairing::request_logger_fairing::RequestLogger;

#[macro_use]
extern crate diesel;

pub mod config;
mod database;
pub mod fairing;
mod models;
mod routes;
mod schema;
pub mod service;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    // rocket::build()

    let package_root_dir = config::get_package_root_dir();
    if path::Path::new(&package_root_dir).exists() == false {
        std::fs::create_dir_all(&package_root_dir).expect("failed to create package root dir");
    }

    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    rocket::custom(config::from_env())
        .attach(cors_fairing())
        .mount(
            "/api/packages",
            routes![
                routes::packages::list_versions,
                routes::packages::versions_new,
                routes::packages::advisories,
                routes::packages::upload,
                routes::packages::finalize_upload,
            ],
        )
        .mount(
            "/packages",
            FileServer::from(config::get_package_root_dir()),
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(database::sqlite_database::ServerSqliteDatabase::fairing())
        .attach(fairing::cache_fairing::StaticCacheFairing)
        .attach(RequestLogger)
        // .attach(cors_fairing())
        .register("/", catchers![not_found])
}
