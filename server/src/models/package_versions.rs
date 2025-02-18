use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PackageVersionJson {
    pub version: String,

    /// Optional field, false if omitted
    pub retracted: bool,

    pub archive_url: String,

    pub archive_sha256: String,

    /// Pubspec contents as JSON object
    pub pubspec: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,

    /// Optional field, false if omitted
    pub is_discontinued: bool,

    /// Optional field, if isDiscontinued == true
    pub replaced_by: Option<String>,

    /// Optional field, timestamp of the last time the contents of the advisories API changed for this package
    pub advisories_updated: Option<String>,
    
    pub latest: PackageVersionJson,

    pub versions: Vec<PackageVersionJson>,
}
