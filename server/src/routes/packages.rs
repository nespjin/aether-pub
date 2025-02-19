use crate::config;
use crate::database::package_entity::PackageEntity;
use crate::database::sqlite_database;
use crate::database::{package_dao, package_entity, package_version_dao};
use crate::models::http::{ServerJsonResponder, ServerNoContentResponder};
use crate::models::package_versions::{PackageJson, PackageVersionJson};
use crate::service::package_service;
use diesel::SqliteConnection;
use rocket::form::validate::{len, Len};
use rocket::form::{Form, Shareable};
use rocket::fs::TempFile;
use rocket::http::{ContentType, Header, Status};
use rocket::response::Responder;
use rocket::serde::json::Value;
use rocket::tokio::fs::remove_file;
use rocket::{get, post, FromForm, Response, State};
use serde_json::json;
use std::fs::File;
use std::io::{Cursor, Read};

/// List all versions of a package
#[get("/<package>")]
pub fn list_versions(package: &str) -> String {
    let mut json = serde_json::json!("{}").to_string();

    let connection = &mut sqlite_database::establish_connection();

    let package_entity = package_dao::find_by_name(connection, package);

    if let Ok(package_entity) = &package_entity {
        println!("Found package: {:?}", package_entity);

        let last_version_entity =
            package_version_dao::find_by_version(connection, &package_entity.latest_version)
                .unwrap();

        let package_file_path = package_service::get_package_file_path(
            &package_entity.name.as_str(),
            &last_version_entity.version,
        )
        .unwrap();
        let archive_url = config::get_package_upload_url()
            + "/"
            + &package_entity.name
            + "/"
            + &last_version_entity.version;

        let sha256_file_path = package_service::get_sha256_file_path(
            &package_entity.name.as_str(),
            &last_version_entity.version,
        )
        .unwrap();

        let archive_sha256 = &mut String::new();
        let _ = File::open(&sha256_file_path)
            .unwrap()
            .read_to_string(archive_sha256)
            .unwrap();

        json = serde_json::to_string(&package_entity.to_json(
            &last_version_entity.to_json(&package_file_path, &archive_sha256),
            vec![last_version_entity.to_json(&package_file_path, &archive_sha256)],
        ))
        .unwrap();

        let package_version_entities =
            package_version_dao::find_all_by_package_name(connection, package);
        if let Ok(entities) = package_version_entities {}
    } else {
        println!("No package found");
    }

    serde_json::to_string(&json).unwrap()
}

#[get("/versions/new")]
pub fn versions_new() -> ServerJsonResponder {
    let body = serde_json::to_string(&serde_json::json!({
        "url": &config::get_package_upload_url(),
        "fields": {}
    }))
    .unwrap();

    ServerJsonResponder::new(&body)
}

#[get("/<package>/advisories")]
pub fn advisories(package: &str) -> Value {
    serde_json::json!(
        {
            "advisories": [],
            "advisoriesUpdated": ""
        }
    )
}

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/upload", data = "<data>")]
pub async fn upload(mut data: Form<Upload<'_>>) -> ServerNoContentResponder {
    let path = data.file.path().unwrap().to_str().unwrap().to_string();

    let url = if let Some(_) = package_service::save_package_and_sha256_file(&path) {
        config::get_finalize_upload_url()
    } else {
        "".to_string()
    };

    ServerNoContentResponder::new(&url)
}

#[get("/finalize-upload")]
pub fn finalize_upload() -> ServerJsonResponder {
    let body = serde_json::to_string(&serde_json::json!({
        "success": {
            "message": "Upload success"
        }
    }))
    .unwrap();

    ServerJsonResponder::new(&body)
}
