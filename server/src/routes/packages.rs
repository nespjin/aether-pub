use crate::models::package_versions::{PackageVersionJson, PackageVersionsJson};
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use serde::Deserialize;
use std::collections::HashMap;
use std::iter::Map;

/// List all versions of a package
#[get("/<package>")]
pub fn list_versions(package: &str) -> String {
    println!("Listing versions of package: {}", package);

    let package_version_json = PackageVersionJson {
        version: "0.1.0".to_string(),
        retracted: false,
        archive_url: "https://example.com/archive.tar.gz".to_string(),
        archive_sha256: "deadbeef".to_string(),
        pubspec: Value::Null,
    };

    let json = PackageVersionsJson {
        name: package.to_string(),
        is_discontinued: false,
        replaced_by: None,
        advisories_updated: None,
        latest: package_version_json,
        versions: vec![],
    };

    serde_json::to_string(&json).unwrap()
}
