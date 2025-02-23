use crate::models::package_version::PackageVersion;

pub struct Package<'a> {
    pub name: &'a str,

    pub is_discontinued: bool,

    pub replaced_by: Option<String>,

    pub advisories_updated: Option<String>,

    pub latest: &'a PackageVersion,

    pub versions: &'a Vec<PackageVersion>,
}
