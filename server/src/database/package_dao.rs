use crate::database::package_entity::PackageEntity;
use crate::schema::package;
use crate::schema::package::dsl::*;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use rocket_sync_db_pools::diesel::SqliteConnection;

pub(crate) fn upsert(
    conn: &mut SqliteConnection,
    entity: &PackageEntity,
) -> QueryResult<PackageEntity> {
    diesel::insert_into(package::table)
        .values(entity)
        .on_conflict(name)
        .do_update()
        .set(entity)
        .returning(PackageEntity::as_returning())
        .get_result(conn)
    // .expect("Error inserting new package")
}

// pub(crate) fn update(
//     conn: &mut SqliteConnection,
//     package_id: i32,
//     entity: &PackageEntity,
// ) -> QueryResult<PackageEntity> {
//     diesel::update(package::table.filter(id.eq(package_id)))
//         .set(entity)
//         .returning(PackageEntity::as_returning())
//         .get_result(conn)
// }

pub(crate) fn upsert_by_name(
    conn: &mut SqliteConnection,
    name_str: &str,
    entity: &PackageEntity,
) -> QueryResult<PackageEntity> {
    diesel::update(package::table.filter(name.eq(name_str)))
        .set(entity)
        .returning(PackageEntity::as_returning())
        .get_result(conn)
}

// pub(crate) fn find(conn: &mut SqliteConnection, package_id: i32) -> QueryResult<PackageEntity> {
//     package::table
//         .filter(id.eq(package_id))
//         .select(PackageEntity::as_select())
//         .first(conn)
// }

pub(crate) fn find_by_name(
    conn: &mut SqliteConnection,
    name_str: &str,
) -> QueryResult<PackageEntity> {
    package::table
        .filter(name.eq(name_str))
        .select(PackageEntity::as_select())
        .first(conn)
}

pub(crate) fn find_all(conn: &mut SqliteConnection) -> QueryResult<Vec<PackageEntity>> {
    package::table.select(PackageEntity::as_select()).load(conn)
}

// pub(crate) fn delete(conn: &mut SqliteConnection, package_id: i32) -> QueryResult<usize> {
//     diesel::delete(package::table.filter(id.eq(package_id))).execute(conn)
// }

pub(crate) fn delete_by_name(conn: &mut SqliteConnection, name_str: &str) -> QueryResult<usize> {
    diesel::delete(package::table.filter(name.eq(name_str))).execute(conn)
}

pub(crate) fn delete_all(conn: &mut SqliteConnection) -> QueryResult<usize> {
    diesel::delete(package::table).execute(conn)
}
