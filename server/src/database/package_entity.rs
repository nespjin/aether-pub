use crate::models::package_versions::{PackageJson, PackageVersionJson};
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::package)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct PackageEntity {
    // pub id: i32,
    pub name: String,
    pub is_discontinued: bool,
    pub replaced_by: Option<String>,
    pub advisories_updated: Option<String>,
    pub latest_version: String,
    pub created_time: i32,
    pub updated_time: i32,
}

impl PackageEntity {
    pub fn new_with_pubspec(pubspec: &serde_json::Value, latest_version: &str) -> Self {
        Self {
            // id: 0,
            name: pubspec["name"].as_str().unwrap().to_string(),
            is_discontinued: false,
            replaced_by: None,
            advisories_updated: None,
            latest_version: latest_version.to_string(),
            created_time: 0,
            updated_time: 0,
        }
    }

    pub fn to_json(
        &self,
        latest_version: &PackageVersionJson,
        versions: Vec<PackageVersionJson>,
    ) -> PackageJson {
        PackageJson {
            name: self.name.clone(),
            is_discontinued: self.is_discontinued,
            replaced_by: self.replaced_by.clone(),
            advisories_updated: self.advisories_updated.clone(),
            latest: latest_version.clone(),
            versions: versions,
        }
    }
}
