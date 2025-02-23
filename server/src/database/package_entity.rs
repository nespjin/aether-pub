use crate::models::package::Package;
use crate::models::package_version::PackageVersion;
use chrono::Utc;
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
    // The seconds since epoch 1970-01-01T00:00:00Z UTC
    pub created_time: i32,
    // The seconds since epoch 1970-01-01T00:00:00Z UTC
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
            created_time: Utc::now().timestamp() as i32,
            updated_time: Utc::now().timestamp() as i32,
        }
    }

    pub fn as_external_model(
        &self,
        latest: PackageVersion,
        versions: Vec<PackageVersion>,
    ) -> Package {
        Package {
            name: &self.name,
            is_discontinued: self.is_discontinued,
            replaced_by: self.replaced_by.clone(),
            advisories_updated: self.advisories_updated.clone(),
            latest,
            versions,
        }
    }

    pub fn copy(origin: &PackageEntity) -> Self {
        Self {
            name: origin.name.clone(),
            is_discontinued: origin.is_discontinued,
            replaced_by: origin.replaced_by.clone(),
            advisories_updated: origin.advisories_updated.clone(),
            latest_version: origin.latest_version.clone(),
            created_time: origin.created_time,
            updated_time: origin.updated_time,
        }
    }
}
