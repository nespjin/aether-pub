use crate::database::any_connection::AnyConnection;
use crate::database::package_entity::PackageEntity;
use crate::schema::package;
use crate::schema::package::dsl::*;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

pub(crate) fn upsert(
    conn: &mut AnyConnection,
    entity: &PackageEntity,
) -> QueryResult<PackageEntity> {
    match conn {
        AnyConnection::Postgresql(ref mut conn) => diesel::insert_into(package::table)
            .values(entity)
            .on_conflict(name)
            .do_update()
            .set(entity)
            .returning(PackageEntity::as_returning())
            .get_result(conn),
        AnyConnection::Mysql(ref mut conn) => conn.transaction(|conn| {
            diesel::insert_or_ignore_into(package::table)
                .values(entity)
                // .on_conflict(name)
                // .do_update()
                // .set(entity)
                .execute(conn)?;

            package::table
                .select(PackageEntity::as_select())
                .first(conn)
        }),
        AnyConnection::Sqlite(ref mut conn) => diesel::insert_or_ignore_into(package::table)
            .values(entity)
            .on_conflict(name)
            .do_update()
            .set(entity)
            .returning(PackageEntity::as_returning())
            .get_result(conn),
    }
    // .returning(PackageEntity::as_returning())
    // .get_result(conn)
    // .expect("Error inserting new package")
}

// pub(crate) fn update(
//     conn: &mut AnyConnection,
//     package_id: i32,
//     entity: &PackageEntity,
// ) -> QueryResult<PackageEntity> {
//     diesel::update(package::table.filter(id.eq(package_id)))
//         .set(entity)
//         .returning(PackageEntity::as_returning())
//         .get_result(conn)
// }

pub(crate) fn upsert_by_name(
    conn: &mut AnyConnection,
    name_str: &str,
    entity: &PackageEntity,
) -> QueryResult<usize> {
    diesel::update(package::table.filter(name.eq(name_str)))
        .set(entity)
        .execute(conn)
    // .returning(PackageEntity::as_returning())
    // .get_result(conn)
}

// pub(crate) fn find(conn: &mut AnyConnection, package_id: i32) -> QueryResult<PackageEntity> {
//     package::table
//         .filter(id.eq(package_id))
//         .select(PackageEntity::as_select())
//         .first(conn)
// }

pub(crate) fn query_packages(
    conn: &mut AnyConnection,
    keyword: &str,
    page_size: u32,
    page: u32,
) -> QueryResult<Vec<PackageEntity>> {
    let is_query_all_packages = keyword.is_empty();
    let is_query_all_pages = page_size == 0;

    if is_query_all_packages && is_query_all_pages {
        return find_all(conn);
    }

    if is_query_all_packages {
        return package::table
            .select(PackageEntity::as_select())
            .limit(page_size as i64)
            .offset((page * page_size) as i64)
            .load::<PackageEntity>(conn);
    }

    package::table
        .filter(name.like(format!("%{}%", keyword)))
        .select(PackageEntity::as_select())
        .limit(page_size as i64)
        .offset((page * page_size) as i64)
        .load::<PackageEntity>(conn)
}

pub(crate) fn find_by_name(conn: &mut AnyConnection, name_str: &str) -> QueryResult<PackageEntity> {
    package::table
        .filter(name.eq(name_str))
        .select(PackageEntity::as_select())
        .first(conn)
}

pub(crate) fn find_all(conn: &mut AnyConnection) -> QueryResult<Vec<PackageEntity>> {
    package::table.select(PackageEntity::as_select()).load(conn)
}

// pub(crate) fn delete(conn: &mut AnyConnection, package_id: i32) -> QueryResult<usize> {
//     diesel::delete(package::table.filter(id.eq(package_id))).execute(conn)
// }

pub(crate) fn delete_by_name(conn: &mut AnyConnection, name_str: &str) -> QueryResult<usize> {
    diesel::delete(package::table.filter(name.eq(name_str))).execute(conn)
}

pub(crate) fn delete_all(conn: &mut AnyConnection) -> QueryResult<usize> {
    diesel::delete(package::table).execute(conn)
}
