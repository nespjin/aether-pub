use dotenv::dotenv;
use rocket::{catch, catchers, launch, routes};
use rocket::serde::json::json;
use rocket_cors::{Cors, CorsOptions};
use serde_json::Value;

mod models;
mod routes;

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
    // dotenv().ok();
    rocket::build()
        .attach(cors_fairing())
        .mount("/api/packages", routes![routes::packages::list_versions])
        // .attach(cors_fairing())
        .register("/", catchers![not_found])
}