use crate::database::any_connection::AnyConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket_sync_db_pools::diesel;
use std::env;

// #[database("aether-pub-server-sqlite")]
// pub(crate) struct ServerDatabase(AnyConnection);

// static DATABASE_CONNECTION: OnceLock<SqliteConnection> = OnceLock::new();
//
// pub(crate) fn get_connection() -> &'static SqliteConnection {
//     DATABASE_CONNECTION.get_or_init(|| establish_connection());
//     DATABASE_CONNECTION.get().unwrap()
// }

pub fn establish_connection() -> AnyConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AnyConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
