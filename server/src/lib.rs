#[macro_use]
extern crate diesel;
extern crate rocket;
use dotenv::dotenv;
use log::LevelFilter;
use rocket::fs::{relative, FileServer};
use rocket::{catch, catchers, launch, routes};
use std::path;

use crate::fairing::request_logger_fairing::RequestLogger;
use rocket::serde::json::json;
use rocket_cors::{Cors, CorsOptions};
use serde_json::Value;

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

    if cfg!(debug_assertions) {
        env_logger::Builder::from_default_env()
            .filter_level(LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(LevelFilter::Error)
            .init();
    }

    let app = rocket::custom(config::from_env())
        // let app = rocket::build()
        .attach(cors_fairing())
        .mount(
            "/api/packages",
            routes![
                routes::packages::package_info,
                routes::packages::list_packages,
                routes::packages::package_readme,
                routes::packages::package_changelog,
                routes::packages::package_example,
                routes::packages::package_versions,
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
        .register("/", catchers![not_found]);
    // .attach(cors_fairing())

    if cfg!(debug_assertions) {
        app.attach(RequestLogger)
    } else {
        app
    }
}
