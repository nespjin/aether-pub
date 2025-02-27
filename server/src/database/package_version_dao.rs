use crate::database::any_connection::AnyConnection;
use crate::database::package_version_entity::PackageVersionEntity;
use crate::schema::package_version;
use crate::schema::package_version::dsl::*;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

pub(crate) fn upsert(
    conn: &mut AnyConnection,
    entity: &PackageVersionEntity,
) -> QueryResult<PackageVersionEntity> {
    match conn {
        AnyConnection::Postgresql(ref mut conn) => diesel::insert_into(package_version::table)
            .values(entity)
            .on_conflict(version)
            .do_update()
            .set(entity)
            .returning(PackageVersionEntity::as_returning())
            .get_result(conn),
        AnyConnection::Mysql(ref mut conn) => conn.transaction(|conn| {
            diesel::insert_or_ignore_into(package_version::table)
                .values(entity)
                // .on_conflict(name)
                // .do_update()
                // .set(entity)
                .execute(conn)?;

            package_version::table
                .select(PackageVersionEntity::as_select())
                .first(conn)
        }),
        AnyConnection::Sqlite(ref mut conn) => {
            diesel::insert_or_ignore_into(package_version::table)
                .values(entity)
                .on_conflict(version)
                .do_update()
                .set(entity)
                .returning(PackageVersionEntity::as_returning())
                .get_result(conn)
        }
    }
    // .returning(PackageEntity::as_returning())
    // .get_result(conn)
    // .expect("Error inserting new package")
}

// pub(crate) fn update(
//     conn: &mut AnyConnection,
//     version_id: i32,
//     entity: &PackageVersionEntity,
// ) -> QueryResult<PackageVersionEntity> {
//     diesel::update(package_version::table.filter(id.eq(version_id)))
//         .set(entity)
//         .returning(PackageVersionEntity::as_returning())
//         .get_result(conn)
// }

pub(crate) fn update_by_version(
    conn: &mut AnyConnection,
    version_str: &str,
    entity: &PackageVersionEntity,
) -> QueryResult<usize> {
    diesel::update(package_version::table.filter(version.eq(version_str)))
        .set(entity)
        .execute(conn)
        // .returning(PackageVersionEntity::as_returning())
        // .get_result(conn)
}
// pub(crate) fn find(
//     conn: &mut AnyConnection,
//     version_id: i32,
// ) -> QueryResult<PackageVersionEntity> {
//     package_version::table
//         .filter(id.eq(version_id))
//         .select(PackageVersionEntity::as_select())
//         .first(conn)
// }

pub(crate) fn find_by_version(
    conn: &mut AnyConnection,
    version_str: &str,
) -> QueryResult<PackageVersionEntity> {
    package_version::table
        .filter(version.eq(version_str))
        .select(PackageVersionEntity::as_select())
        .first(conn)
}

pub(crate) fn find_all_by_package_name(
    conn: &mut AnyConnection,
    pkg_name: &str,
) -> QueryResult<Vec<PackageVersionEntity>> {
    package_version::table
        .filter(package_name.eq(pkg_name))
        .select(PackageVersionEntity::as_select())
        .load(conn)
}

pub(crate) fn find_all(conn: &mut AnyConnection) -> QueryResult<Vec<PackageVersionEntity>> {
    package_version::table
        .select(PackageVersionEntity::as_select())
        .load(conn)
}

// pub(crate) fn delete(conn: &mut AnyConnection, version_id: i32) -> QueryResult<usize> {
//     diesel::delete(package_version::table.filter(id.eq(version_id))).execute(conn)
// }

pub(crate) fn delete_by_version(conn: &mut AnyConnection, version_str: &str) -> QueryResult<usize> {
    diesel::delete(package_version::table.filter(version.eq(version_str))).execute(conn)
}

pub(crate) fn delete_all(conn: &mut AnyConnection) -> QueryResult<usize> {
    diesel::delete(package_version::table).execute(conn)
}
