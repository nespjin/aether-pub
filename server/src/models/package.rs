use crate::models::package_version::PackageVersion;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,

    pub is_discontinued: bool,

    pub replaced_by: Option<String>,

    pub advisories_updated: Option<String>,

    pub latest: PackageVersion,

    pub versions: Vec<PackageVersion>,
}
