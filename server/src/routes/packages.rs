use crate::config;
use crate::routes::http::{ServerJsonResponder, ServerNoContentResponder};
use crate::routes::package_response_data::PackageResponseData;
use crate::service::package_service;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Value;
use rocket::{get, post, FromForm};
use crate::routes::package_version_response_data::PackageVersionResponseData;

/// List all versions of a package
#[get("/<package>")]
pub fn package_info(package: &str) -> ServerJsonResponder {
    match package_service::query_package(package, true) {
        Some(package) => {
            let data = PackageResponseData::from_model(&package);
            ServerJsonResponder::new(&serde_json::to_string(&data).unwrap())
        }
        None => ServerJsonResponder::new("{}"),
    }
}

#[get("/<package>/readme")]
pub fn package_readme(package: &str) -> ServerJsonResponder {
    match package_service::get_package_readme(package) {
        Some(readme) => ServerJsonResponder::new(&readme),
        None => ServerJsonResponder::new(""),
    }
}

#[get("/<package>/changelog")]
pub fn package_changelog(package: &str) -> ServerJsonResponder {
    match package_service::get_package_changelog(package) {
        Some(changelog) => ServerJsonResponder::new(&changelog),
        None => ServerJsonResponder::new(""),
    }
}

#[get("/<package>/example")]
pub fn package_example(package: &str) -> ServerJsonResponder {
    match package_service::get_package_example(package) {
        Some(example) => ServerJsonResponder::new(&example),
        None => ServerJsonResponder::new(""),
    }
}

#[get("/<package>/versions")]
pub fn package_versions(package: &str) -> ServerJsonResponder {
    let versions = package_service::query_package_versions(package);
    let data = versions
        .iter()
        .map(PackageVersionResponseData::from_model)
        .collect::<Vec<PackageVersionResponseData>>();

    ServerJsonResponder::new(&serde_json::to_string(&data).unwrap())
}

#[get("/packages-all?<keyword>&<page_size>&<page>&<is_query_all_versions>")]
pub fn list_packages(
    keyword: Option<&str>,
    page_size: Option<u32>,
    page: Option<u32>,
    is_query_all_versions: Option<bool>,
) -> ServerJsonResponder {
    let keyword = if let Some(keyword) = keyword {
        keyword
    } else {
        ""
    };

    let page_size = if let Some(page_size) = page_size {
        page_size
    } else {
        0
    };

    let page = if let Some(page) = page { page } else { 0 };

    let is_query_all_versions = if let Some(is_query_all_versions) = is_query_all_versions {
        is_query_all_versions
    } else {
        false
    };

    match package_service::query_packages(keyword, page_size, page, is_query_all_versions) {
        Some(packages) => {
            let data = packages
                .iter()
                .map(PackageResponseData::from_model)
                .collect::<Vec<PackageResponseData>>();

            ServerJsonResponder::new(&serde_json::to_string(&data).unwrap())
        }
        None => ServerJsonResponder::new("[]"),
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
