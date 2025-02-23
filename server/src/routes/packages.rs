use crate::config;
use crate::database::sqlite_database;
use crate::database::{package_dao, package_version_dao};
use crate::routes::http::{ServerJsonResponder, ServerNoContentResponder};
use crate::routes::package_response_data::PackageResponseData;
use crate::service::package_service;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Value;
use rocket::{get, post, FromForm};
use std::fs::File;
use std::io::Read;

/// List all versions of a package
#[get("/<package>")]
pub fn list_versions(package: &str) -> ServerJsonResponder {
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
        let archive_url = format!("{}/{}", "http://127.0.0.1:8000", &package_file_path);

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

        json = serde_json::to_string(&PackageResponseData::from_model(
            &package_entity.as_external_model(
                &last_version_entity.as_external_model(&archive_url, &archive_sha256),
                &vec![last_version_entity.as_external_model(&archive_url, &archive_sha256)],
            ),
        ))
        .unwrap();

        let package_version_entities =
            package_version_dao::find_all_by_package_name(connection, package);
        if let Ok(_entities) = package_version_entities {}
    } else {
        println!("No package found");
    }

    // serde_json::to_string(&json).unwrap()
    ServerJsonResponder::new(&json)
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
pub(crate) struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/upload", data = "<data>")]
pub async fn upload(mut data: Form<Upload<'_>>) -> ServerNoContentResponder {
    let path = data.file.path().unwrap().to_str().unwrap().to_string();

    let url = if let Some(_) = package_service::save_new_package_version_with_tar_file(&path) {
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
