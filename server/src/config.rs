use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

/// Debug only secret for JWT encoding & decoding.
const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &'static str = "Token ";

pub fn check() {
    // Check for required environment variables
    dotenv::dotenv().ok();
    let keys = ["PACKAGE_UPLOAD_URL", "FINALIZE_UPLOAD_URL"];
    for key in keys.iter() {
        if env::var(key).is_err() {
            panic!("{} environment variable not found", key);
        }
    }
}

pub fn get_package_root_dir() -> String {
    dotenv::dotenv().ok();
    env::var("PACKAGE_ROOT_DIR").unwrap_or_else(|_| "packages".to_string())
}

pub fn get_package_upload_url() -> String {
    dotenv::dotenv().ok();
    env::var("PACKAGE_UPLOAD_URL").unwrap()
}

pub fn get_finalize_upload_url() -> String {
    dotenv::dotenv().ok();
    env::var("FINALIZE_UPLOAD_URL").unwrap()
}

pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async move {
            // Rocket doesn't expose it's own secret_key, so we use our own here.
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    SECRET.to_string()
                    // panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            rocket.manage(AppState {
                secret: secret.into_bytes(),
            })
        })
    }
}

/// Create rocket config from environment variables
pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_sqlite_pool", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("databases", databases))
}
