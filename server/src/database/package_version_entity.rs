use crate::models::package_versions::PackageVersionJson;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::package_version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PackageVersionEntity {
    // pub id: i32,
    pub version: String,
    pub retracted: bool,
    pub archive_url: String,
    pub archive_sha256: String,
    pub pubspec_json: String,
    pub package_name: String,
    pub created_time: i32,
    pub updated_time: i32,
}

impl PackageVersionEntity {
    pub fn new_with_pubspec(pubspec: &serde_json::Value, package_name: &str) -> Self {
        Self {
            version: pubspec["version"].as_str().unwrap().to_string(),
            retracted: false,
            archive_url: String::new(),
            archive_sha256: String::new(),
            pubspec_json: serde_json::to_string(&pubspec).unwrap(),
            package_name: package_name.to_string(),
            created_time: 0,
            updated_time: 0,
        }
    }

    pub fn to_json(&self, archive_url: &str, archive_sha256: &str) -> PackageVersionJson {
        PackageVersionJson {
            version: self.version.to_string(),
            retracted: self.retracted,
            archive_url: archive_url.to_string(),
            archive_sha256: archive_sha256.to_string(),
            pubspec: serde_json::from_str(&self.pubspec_json).unwrap(),
        }
    }
}
