use crate::models::package_versions::PackageVersionJson;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::package)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct PackageEntity {
    pub id: i32,
    pub name: String,
    pub is_discontinued: bool,
    pub replaced_by: Option<String>,
    pub advisories_updated: Option<String>,
    pub latest_version_id: i32,
    pub created_time: i32,
    pub updated_time: i32,
}

impl PackageEntity {
    pub fn new(json: &PackageVersionJson) -> PackageEntity {
        PackageEntity {
            id: 0,
            name: json.version.clone(),
            is_discontinued: json.retracted,
            replaced_by: None,
            advisories_updated: None,
            latest_version_id: 0,
            created_time: 0,
            updated_time: 0,
        }
    }
}
