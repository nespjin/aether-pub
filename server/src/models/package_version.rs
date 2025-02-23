pub struct PackageVersion {
    pub version: String,

    pub retracted: bool,

    pub archive_url: String,

    pub archive_sha256: String,

    pub pubspec: serde_json::Value,
}
