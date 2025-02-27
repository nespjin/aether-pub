use diesel::prelude::*;

#[derive(MultiConnection)]
pub enum AnyConnection {
    Postgresql(diesel::PgConnection),
    Mysql(diesel::MysqlConnection),
    Sqlite(diesel::SqliteConnection),
}
