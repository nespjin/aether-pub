use crate::database::package_version_entity::PackageVersionEntity;
use crate::schema::package::name;
use crate::schema::package_version;
use crate::schema::package_version::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use rocket_sync_db_pools::diesel::SqliteConnection;

pub(crate) fn upsert(
    conn: &mut SqliteConnection,
    entity: &PackageVersionEntity,
) -> QueryResult<PackageVersionEntity> {
    diesel::insert_into(package_version::table)
        .values(entity)
        .on_conflict(version)
        .do_update()
        .set(entity)
        .returning(PackageVersionEntity::as_returning())
        .get_result(conn)
    // .expect("Error inserting new package")
}

// pub(crate) fn update(
//     conn: &mut SqliteConnection,
//     version_id: i32,
//     entity: &PackageVersionEntity,
// ) -> QueryResult<PackageVersionEntity> {
//     diesel::update(package_version::table.filter(id.eq(version_id)))
//         .set(entity)
//         .returning(PackageVersionEntity::as_returning())
//         .get_result(conn)
// }

pub(crate) fn update_by_version(
    conn: &mut SqliteConnection,
    version_str: &str,
    entity: &PackageVersionEntity,
) -> QueryResult<PackageVersionEntity> {
    diesel::update(package_version::table.filter(version.eq(version_str)))
        .set(entity)
        .returning(PackageVersionEntity::as_returning())
        .get_result(conn)
}
// pub(crate) fn find(
//     conn: &mut SqliteConnection,
//     version_id: i32,
// ) -> QueryResult<PackageVersionEntity> {
//     package_version::table
//         .filter(id.eq(version_id))
//         .select(PackageVersionEntity::as_select())
//         .first(conn)
// }

pub(crate) fn find_by_version(
    conn: &mut SqliteConnection,
    version_str: &str,
) -> QueryResult<PackageVersionEntity> {
    package_version::table
        .filter(version.eq(version_str))
        .select(PackageVersionEntity::as_select())
        .first(conn)
}

pub(crate) fn find_all_by_package_name(
    conn: &mut SqliteConnection,
    pkg_name: &str,
) -> QueryResult<Vec<PackageVersionEntity>> {
    package_version::table
        .filter(package_name.eq(pkg_name))
        .select(PackageVersionEntity::as_select())
        .load(conn)
}

pub(crate) fn find_all(conn: &mut SqliteConnection) -> QueryResult<Vec<PackageVersionEntity>> {
    package_version::table
        .select(PackageVersionEntity::as_select())
        .load(conn)
}

// pub(crate) fn delete(conn: &mut SqliteConnection, version_id: i32) -> QueryResult<usize> {
//     diesel::delete(package_version::table.filter(id.eq(version_id))).execute(conn)
// }

pub(crate) fn delete_by_version(
    conn: &mut SqliteConnection,
    version_str: &str,
) -> QueryResult<usize> {
    diesel::delete(package_version::table.filter(version.eq(version_str))).execute(conn)
}

pub(crate) fn delete_all(conn: &mut SqliteConnection) -> QueryResult<usize> {
    diesel::delete(package_version::table).execute(conn)
}
