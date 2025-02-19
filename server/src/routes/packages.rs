use crate::database::package_dao;
use crate::database::package_entity::PackageEntity;
use crate::database::sqlite_database;
use crate::models::package_versions::{PackageVersionJson, PackageVersionsJson};
use diesel::SqliteConnection;
use rocket::get;
use rocket::serde::json::Value;

/// List all versions of a package
#[get("/<package>")]
pub fn list_versions(package: &str) -> String {
    println!("Listing versions of package: {:?}", package);

    let package_version_json = PackageVersionJson {
        version: "0.1.0".to_string(),
        retracted: false,
        archive_url: "https://example.com/archive.tar.gz".to_string(),
        archive_sha256: "deadbeef".to_string(),
        pubspec: Value::Null,
    };

    let entity = PackageEntity::new(&package_version_json);

    let json = PackageVersionsJson {
        name: package.to_string(),
        is_discontinued: false,
        replaced_by: None,
        advisories_updated: None,
        latest: package_version_json,
        versions: vec![],
    };

    let connection = &mut sqlite_database::establish_connection();

    let entity = package_dao::find_by_name(connection, package);

    if let Ok(entity) = &entity {
        println!("Found package: {:?}", entity);
    } else {
        println!("No package found");
    }
    serde_json::to_string(&json).unwrap()
}

#[get("/versions/new")]
pub fn versions_new() {}

#[get("/<package>/advisories")]
pub fn advisories(package: &str) -> Value {
    serde_json::json!(
        {
            "advisories": [],
            "advisoriesUpdated": ""
        }
    )
}
