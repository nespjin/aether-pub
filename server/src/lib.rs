use dotenv::dotenv;
use rocket::{catch, catchers, launch, routes};

extern crate rocket;
use rocket::serde::json::json;
use rocket_cors::{Cors, CorsOptions};
use serde_json::Value;

#[macro_use]
extern crate diesel;

mod config;
mod database;
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
    rocket::custom(config::from_env())
        .attach(cors_fairing())
        .mount(
            "/api/packages",
            routes![
                routes::packages::list_versions,
                routes::packages::versions_new,
                routes::packages::advisories,
            ],
        )
        .attach(database::sqlite_database::ServerSqliteDatabase::fairing())
        // .attach(cors_fairing())
        .register("/", catchers![not_found])
}
