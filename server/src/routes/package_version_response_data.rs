use crate::models::package_version::PackageVersion;
use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PackageVersionResponseData {
    pub version: String,

    /// Optional field, false if omitted
    pub retracted: bool,

    pub archive_url: String,

    pub archive_sha256: String,

    /// Pubspec contents as JSON object
    pub pubspec: serde_json::Value,
}

impl PackageVersionResponseData {
    pub fn from_model(model: &PackageVersion) -> PackageVersionResponseData {
        PackageVersionResponseData {
            version: model.version.clone(),
            retracted: model.retracted,
            archive_url: model.archive_url.to_string(),
            archive_sha256: model.archive_sha256.to_string(),
            pubspec: model.pubspec.clone(),
        }
    }
}
