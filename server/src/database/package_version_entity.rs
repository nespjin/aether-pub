use diesel::prelude::*;


#[derive(Insertable, Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::package_version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct PackageVersionEntity {
    pub id: i32,
    pub version: String,
    pub retracted: bool,
    pub archive_url: String,
    pub archive_sha256: String,
    pub pubspec_json: String,
    pub package_id: i32,
    pub created_time: i32,
    pub updated_time: i32,
}
