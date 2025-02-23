use crate::models::package::Package;
use crate::routes::package_version_response_data::PackageVersionResponseData;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageResponseData {
    pub name: String,

    /// Optional field, false if omitted
    pub is_discontinued: bool,

    /// Optional field, if isDiscontinued == true
    pub replaced_by: Option<String>,

    /// Optional field, timestamp of the last time the contents of the advisories API changed for this package
    pub advisories_updated: Option<String>,

    pub latest: PackageVersionResponseData,

    pub versions: Vec<PackageVersionResponseData>,
}

impl PackageResponseData {
    pub fn from_model(model: &Package) -> PackageResponseData {
        PackageResponseData {
            name: model.name.to_string(),
            is_discontinued: model.is_discontinued,
            replaced_by: model.replaced_by.clone().map(|s| s.to_string()),
            advisories_updated: model.advisories_updated.clone(),
            latest: PackageVersionResponseData::from_model(&model.latest),
            versions: model
                .versions
                .iter()
                .map(PackageVersionResponseData::from_model)
                .collect(),
        }
    }
}
