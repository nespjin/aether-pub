use diesel::prelude::*;
use dotenv::dotenv;
use rocket_sync_db_pools::{database, diesel};
use std::env;

#[database("aether-pub-server-sqlite")]
pub(crate) struct ServerSqliteDatabase(SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
