use crate::models::package_version::PackageVersion;
use chrono::Utc;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::package_version)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PackageVersionEntity {
    // pub id: i32,
    pub version: String,
    pub retracted: bool,
    pub pubspec_json: String,
    pub package_name: String,
    // The seconds since epoch 1970-01-01T00:00:00Z UTC
    pub created_time: i32,
    // The seconds since epoch 1970-01-01T00:00:00Z UTC
    pub updated_time: i32,
}

impl PackageVersionEntity {
    pub fn new_with_pubspec(pubspec: &serde_json::Value, retracted: bool) -> Self {
        Self {
            version: pubspec["version"].as_str().unwrap().to_string(),
            retracted,
            pubspec_json: serde_json::to_string(&pubspec).unwrap(),
            package_name: pubspec["name"].as_str().unwrap().to_string(),
            created_time: Utc::now().timestamp() as i32,
            updated_time: Utc::now().timestamp() as i32,
        }
    }

    pub fn as_external_model(&self, archive_url: &str, archive_sha256: &str) -> PackageVersion {
        PackageVersion {
            version: self.version.to_string(),
            retracted: self.retracted,
            archive_url: archive_url.to_string(),
            archive_sha256: archive_sha256.to_string(),
            pubspec: serde_json::from_str(&self.pubspec_json).unwrap(),
        }
    }

    pub fn copy(origin: &PackageVersionEntity) -> Self {
        Self {
            version: origin.version.clone(),
            retracted: origin.retracted,
            pubspec_json: origin.pubspec_json.clone(),
            package_name: origin.package_name.clone(),
            created_time: origin.created_time,
            updated_time: origin.updated_time,
        }
    }
}
