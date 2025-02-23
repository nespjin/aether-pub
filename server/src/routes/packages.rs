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
    match package_service::query_package(package, true) {
        Some(package) => {
            let data = PackageResponseData::from_model(&package);
            ServerJsonResponder::new(&serde_json::to_string(&data).unwrap())
        }
        None => ServerJsonResponder::new("{}"),
    }
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
