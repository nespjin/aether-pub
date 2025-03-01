use crate::config;
use crate::database::package_entity::PackageEntity;
use crate::database::package_version_entity::PackageVersionEntity;
use crate::database::{package_dao, package_version_dao, server_database};
use crate::models::package::Package;
use crate::models::package_version::PackageVersion;
use diesel::Connection;
use log::error;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{BufReader, Read, Seek, Write};
use std::path::Path;
// pub(crate) struct PackageService<'a> {
//     /// The database connection.
//     pub(crate) db: &'a mut SqliteConnection,
// }
//
// impl<'a> PackageService<'a> {
//     pub(crate) fn new(db: &'a mut SqliteConnection) -> Self {
//         Self { db }
//     }
// }
//

pub fn query_package(package_name: &str, is_query_versions: bool) -> Option<Package> {
    let connection = &mut server_database::establish_connection();
    let package_entity = match package_dao::find_by_name(connection, package_name) {
        Ok(entity) => entity,
        Err(e) => {
            error!("get_package: failed to find package by name: {:?}", e);
            return None;
        }
    };
    let latest_version: &str = &package_entity.latest_version;

    let last_version_entity = match package_version_dao::find_by_version(connection, latest_version)
    {
        Ok(entity) => entity,
        Err(e) => {
            error!(
                "get_package: failed to find package version by version: {:?}",
                e
            );
            return None;
        }
    };

    let last_version = match get_package_version_from_entity(&last_version_entity) {
        Some(version) => version,
        None => {
            error!("get_package: failed to get package version from entity");
            return None;
        }
    };

    let version: Option<Vec<PackageVersion>> = if is_query_versions {
        let version_entities =
            match package_version_dao::find_all_by_package_name(connection, package_name) {
                Ok(entities) => entities,
                Err(e) => {
                    error!(
                        "get_package: failed to find all package versions by package name: {:?}",
                        e
                    );
                    return None;
                }
            };

        match version_entities
            .iter()
            .map(|entity| get_package_version_from_entity(entity))
            .collect::<Option<Vec<PackageVersion>>>()
        {
            Some(versions) => Some(versions),
            None => {
                error!("get_package: failed to get package versions from entities");
                None
            }
        }
    } else {
        None
    };

    Some(Package {
        name: package_name.to_string(),
        is_discontinued: false,
        replaced_by: None,
        advisories_updated: None,
        latest: last_version,
        versions: version.unwrap_or(Vec::new()),
    })
}

pub fn query_packages(
    keyword: &str,
    page_size: u32,
    page: u32,
    is_query_all_versions: bool,
) -> Option<Vec<Package>> {
    let connection = &mut server_database::establish_connection();
    connection
        .transaction(|connection| {
            let package_entities =
                match package_dao::query_packages(connection, keyword, page_size, page) {
                    Ok(entities) => entities,
                    Err(e) => {
                        error!("query_packages: failed to query packages: {:?}", e);
                        return Err(e);
                    }
                };

            let mut packages: Vec<Package> = Vec::new();

            for entity in package_entities {
                let package_name: &str = &entity.name;

                if is_query_all_versions {
                    let result =
                        package_version_dao::find_all_by_package_name(connection, package_name);
                    let package_version_entities = result.unwrap_or_else(|e| {
                        error!(
                            "get_package: failed to find all package versions by package name: {:?}",
                            e
                        );
                        vec![]
                    });
                    let versions = package_version_entities
                        .iter()
                        .map(|entity| get_package_version_from_entity(entity))
                        .collect::<Option<Vec<PackageVersion>>>()
                        .unwrap_or_else(|| {
                            error!("get_package: failed to get package versions from entities");
                            vec![]
                        });

                    let last_version = match versions
                        .iter()
                        .find(|version| version.version == entity.latest_version)
                    {
                        Some(version) => version,
                        None => {
                            error!("get_package: failed to find last version");
                            continue;
                        }
                    };

                    let package = entity.as_external_model(last_version.clone(), versions);
                    packages.push(package);
                } else {
                    let last_version_opt = match package_version_dao::find_by_version(
                        connection,
                        &entity.latest_version,
                    ) {
                        Ok(entity) => get_package_version_from_entity(&entity),
                        Err(e) => {
                            error!(
                                "get_package: failed to find package version by version: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    let last_version = match last_version_opt {
                        Some(version) => version,
                        None => {
                            error!("get_package: failed to get package version from entity");
                            continue;
                        }
                    };

                    let package = entity.as_external_model(last_version, vec![]);
                    packages.push(package);
                }
            }

            Ok(packages)
        })
        .map_or(None, |packages| Some(packages))
}
pub fn get_package_details() {}

pub fn get_package_readme(package_name: &str) -> Option<String> {
    let Some(package_file_path) = get_package_file_path_from_package_name(package_name) else {
        error!("get_package_readme: failed to get package file path from package name");
        return None;
    };

    parse_readme_from_tar_gz(&package_file_path)
}

pub fn get_package_changelog(package_name: &str) -> Option<String> {
    let Some(package_file_path) = get_package_file_path_from_package_name(package_name) else {
        error!("get_package_changelog: failed to get package file path from package name");
        return None;
    };

    parse_changelog_from_tar_gz(&package_file_path)
}

pub fn get_package_example(package_name: &str) -> Option<String> {
    let Some(package_file_path) = get_package_file_path_from_package_name(package_name) else {
        error!("get_package_example: failed to get package file path from package name");
        return None;
    };

    parse_example_from_tar_gz(&package_file_path)
}

pub fn get_package_installing() {}

pub fn query_package_versions(package_name: &str) -> Vec<PackageVersion> {
    let connection = &mut server_database::establish_connection();
    connection
        .transaction(|connection| {
            let package_versions =
                match package_version_dao::find_all_by_package_name(connection, package_name) {
                    Ok(entities) => entities,
                    Err(e) => {
                        error!(
                            "get_package_versions: failed to query package versions: {:?}",
                            e
                        );
                        return Err(e);
                    }
                };

            let versions = package_versions
                .iter()
                .map(|entity| get_package_version_from_entity(entity))
                .collect::<Option<Vec<PackageVersion>>>()
                .unwrap_or_else(|| {
                    error!("get_package_versions: failed to get package versions from entities");
                    vec![]
                });
            Ok(versions)
        })
        .map_or(vec![], |versions| versions)
}

pub fn save_new_package_version_with_tar_file(
    package_tmp_file_path: &String,
) -> Option<(String, String)> {
    let Ok(mut package_tmp_file) = File::open(package_tmp_file_path) else {
        error!("save_package_and_sha256_file: failed to open package tmp file");
        return None;
    };

    let Some(pubspec) = parse_pubspec_from_tar_gz(&package_tmp_file) else {
        error!("save_package_and_sha256_file: failed to parse pubspec.yaml");
        return None;
    };

    // Reset file pointer to the beginning for subsequent reads
    if let Err(_) = package_tmp_file.seek(std::io::SeekFrom::Start(0)) {
        error!("save_package_and_sha256_file: failed to reset file pointer");
        return None;
    }

    let (Some(package_name), Some(package_version)) =
        (pubspec["name"].as_str(), pubspec["version"].as_str())
    else {
        error!("save_package_and_sha256_file: failed to extract package name and version");
        return None;
    };

    let Some(package_file_path) = get_package_file_path(package_name, package_version) else {
        error!("save_package_and_sha256_file: failed to get package file path");
        return None;
    };

    let Some(sha256_file_path) = get_sha256_file_path(package_name, package_version) else {
        error!("save_package_and_sha256_file: failed to get sha256 file path");
        return None;
    };

    // error!(
    //     "save_package_and_sha256_file: package_file_path: {:?}",
    //     &package_file_path
    // );
    // error!(
    //     "save_package_and_sha256_file: sha256_file_path: {:?}",
    //     &sha256_file_path
    // );

    // Ensure the parent directories for the package and SHA256 files exist
    Path::new(&package_file_path).parent().map(|parent| {
        if !parent.exists() {
            if let Err(_) = create_dir_all(parent) {
                error!("save_package_and_sha256_file: failed to create package directory");
            }
        }
    });

    Path::new(&sha256_file_path).parent().map(|parent| {
        if !parent.exists() {
            if let Err(_) = create_dir_all(parent) {
                error!("save_package_and_sha256_file: failed to create sha256 directory");
            }
        }
    });

    // Create the package file and SHA256 file
    let Ok(mut package_file) = File::create(&package_file_path) else {
        error!("save_package_and_sha256_file: failed to create package file");
        return None;
    };

    let Ok(mut sha256_file) = File::create(&sha256_file_path) else {
        remove_package_and_sha256_file(&package_file_path, "");
        error!("save_package_and_sha256_file: failed to create sha256 file");
        return None;
    };

    let mut sha256 = Sha256::default();
    let mut package_tmp_file_reader = BufReader::new(&package_tmp_file);

    let mut buffer = [0; 1024];
    let mut package_file_size = 0;

    // Read the temporary package file, calculate its SHA256 hash, and write to the package file
    loop {
        if let Ok(bytes_read) = package_tmp_file_reader.read(&mut buffer) {
            error!("save_package_and_sha256_file: bytes_read: {:?}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            let _ = &sha256.update(&buffer[..bytes_read]);
            package_file_size += bytes_read;

            if let Err(_) = package_file.write(&buffer[..bytes_read]) {
                remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
                error!("save_package_and_sha256_file: failed to write package file");
                return None;
            }
        } else {
            remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
            error!("save_package_and_sha256_file: failed to read tmp package file");
            return None;
        }
    }

    if package_file_size == 0 {
        remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
        error!("save_package_and_sha256_file: package file size is 0");
        return None;
    }

    // Finalize the SHA256 hash and convert it to a hexadecimal string
    let hash_code = sha256.finalize();
    let hash_hex = format!("{:x}", hash_code);

    // Calculate the file sha256 hash
    // let mut sha256 = Sha256::default();
    // sha256.update(buf);
    // let hash_code = sha256.finalize();
    // let hash_hex = format!("{:x}", hash_code);

    error!("save_package_and_sha256_file: file hash: {:?}", &hash_hex);

    // Write the SHA256 hash to the hash file and update the database
    if let Ok(sha256_file_size) = sha256_file.write(&hash_hex.as_bytes()) {
        error!(
            "save_package_and_sha256_file: package_file_size: {:?}, sha256_file_size: {}",
            package_file_size, sha256_file_size
        );
        update_database(package_version, &pubspec);
        Some((package_file_path, sha256_file_path))
    } else {
        error!("save_package_and_sha256_file: failed to write file");
        None
    }
}

pub fn parse_pubspec_from_tar_gz(file: &File) -> Option<serde_json::Value> {
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let mut pubspec = None;

    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(e) => {
            error!("parse_pubspec_from_tar_gz: failed to get entries: {:?}", e);
            return None;
        }
    };

    for entry_result in entries {
        let mut entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                error!("parse_pubspec_from_tar_gz: failed to get entry: {:?}", e);
                continue;
            }
        };

        let entry_path = match entry.path() {
            Ok(path) => path,
            Err(e) => {
                error!(
                    "parse_pubspec_from_tar_gz: failed to get entry path: {:?}",
                    e
                );
                continue;
            }
        };

        if entry_path.ends_with("pubspec.yaml") {
            let mut content = String::new();
            if let Err(e) = entry.read_to_string(&mut content) {
                error!("parse_pubspec_from_tar_gz: failed to read entry: {:?}", e);
                return None;
            }

            // error!("pubspec.yaml content: {}", content);

            // Parse pubspec.yaml to JSON
            pubspec = match serde_yaml::from_str::<serde_json::Value>(&content) {
                Ok(json) => Some(json),
                Err(e) => {
                    error!(
                        "parse_pubspec_from_tar_gz: failed to parse pubspec.yaml: {:?}",
                        e
                    );
                    return None;
                }
            };
            break;
        }
    }
    pubspec
}

pub fn parse_example_from_tar_gz(file_path: &str) -> Option<String> {
    parse_file_from_tar_gz(file_path, |path| {
        // error!("parse_example_from_tar_gz: {:?}", path.to_str());
        path.ends_with("example/lib/main.dart")
            || path.ends_with("example/main.dart")
            || path.ends_with("lib/example.dart")
            || path.ends_with("example.dart")
            || Regex::new(r"(.*)?example.*")
                .unwrap()
                .is_match(path.to_str().unwrap_or(""))
    })
}

pub fn parse_license_from_tar_gz(file_path: &str) -> Option<String> {
    parse_file_from_tar_gz(file_path, |path| {
        path.ends_with("LICENSE") || path.ends_with("LICENSE.md") || path.ends_with("LICENSE.rst")
    })
}

pub fn parse_readme_from_tar_gz(file_path: &str) -> Option<String> {
    parse_file_from_tar_gz(file_path, |path| {
        path.ends_with("README.md") || path.ends_with("README.rst")
    })
}

pub fn parse_changelog_from_tar_gz(file_path: &str) -> Option<String> {
    parse_file_from_tar_gz(file_path, |path| {
        path.ends_with("CHANGELOG.md") || path.ends_with("CHANGELOG.rst")
    })
}

pub fn parse_file_from_tar_gz(
    file_path: &str,
    filter: fn(path: &Cow<Path>) -> bool,
) -> Option<String> {
    let Ok(file) = File::open(file_path) else {
        error!(
            "parse_file_from_tar_gz: failed to open file: {:?}",
            file_path
        );
        return None;
    };

    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));

    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(e) => {
            error!("parse_file_from_tar_gz: failed to get entries: {:?}", e);
            return None;
        }
    };

    for entry_result in entries {
        let mut entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                error!("parse_pubspec_from_tar_gz: failed to get entry: {:?}", e);
                continue;
            }
        };

        let entry_path = match entry.path() {
            Ok(path) => path,
            Err(e) => {
                error!(
                    "parse_pubspec_from_tar_gz: failed to get entry path: {:?}",
                    e
                );
                continue;
            }
        };

        if filter(&entry_path) {
            let mut content = String::new();
            if let Err(e) = entry.read_to_string(&mut content) {
                error!("parse_pubspec_from_tar_gz: failed to read entry: {:?}", e);
                return None;
            }

            return Some(content);
        }
    }
    None
}

fn remove_package_and_sha256_file(package_file_path: &str, sha256_file_path: &str) {
    if !package_file_path.is_empty() {
        if let Err(_) = remove_file(package_file_path) {
            error!("remove_package_and_sha256_file: failed to remove package file");
        }
    }

    if !sha256_file_path.is_empty() {
        if let Err(_) = remove_file(sha256_file_path) {
            error!("remove_package_and_sha256_file: failed to remove sha256 file");
        }
    }
}

fn update_database(package_version: &str, pubspec: &serde_json::Value) {
    let connection = &mut server_database::establish_connection();

    // .unwrap();
    if let Err(e) = package_version_dao::upsert(
        connection,
        &PackageVersionEntity::new_with_pubspec(&pubspec, false),
    ) {
        error!("update_database: failed to upsert package version: {:?}", e);
        return;
    }

    if let Err(e) = package_dao::upsert(
        connection,
        &PackageEntity::new_with_pubspec(&pubspec, package_version),
    ) {
        error!("update_database: failed to upsert package: {:?}", e);
    }
}

#[inline]
pub(crate) fn get_package_file_path(package_name: &str, package_version: &str) -> Option<String> {
    get_file_path(package_name, package_version, get_package_file_name)
}

#[inline]
pub(crate) fn get_sha256_file_path(package_name: &str, package_version: &str) -> Option<String> {
    get_file_path(package_name, package_version, get_sha256_file_name)
}

fn get_file_path(
    package_name: &str,
    package_version: &str,
    get_file_name: fn(&str, &str) -> String,
) -> Option<String> {
    let Some(dir_path) = get_package_dir_path(package_name, package_version) else {
        error!("get_file_path: failed to get package dir path");
        return None;
    };

    let file_name = get_file_name(package_name, package_version);
    let file_path = Path::new(&dir_path).join(&file_name);

    file_path.to_str().map(|path| path.to_string())
}

fn get_package_version_from_entity(entity: &PackageVersionEntity) -> Option<PackageVersion> {
    let package_name: &str = &entity.package_name;
    let package_version: &str = &entity.version;

    let last_version_archive_download_url =
        match get_package_version_archive_download_url(package_name, package_version) {
            Some(path) => path,
            None => {
                error!("get_package: failed to get package file path");
                return None;
            }
        };

    let last_version_sha256 =
        match get_package_version_archive_sha256(package_name, package_version) {
            Some(hash) => hash,
            None => {
                error!("get_package: failed to get package file path");
                return None;
            }
        };

    Some(entity.as_external_model(&last_version_archive_download_url, &last_version_sha256))
}

fn get_package_version_archive_sha256(package_name: &str, package_version: &str) -> Option<String> {
    let sha256_file_path = match get_sha256_file_path(package_name, package_version) {
        Some(path) => path,
        None => {
            error!("get_package: failed to get sha256 file path");
            return None;
        }
    };

    match File::open(&sha256_file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(e) = file.read_to_string(&mut content) {
                error!("get_package: failed to read sha256 file: {:?}", e);
                return None;
            }
            Some(content)
        }
        Err(e) => {
            error!("get_package: failed to open sha256 file: {:?}", e);
            None
        }
    }
}

fn get_package_version_archive_download_url(package_name: &str, version: &str) -> Option<String> {
    match get_package_file_path(package_name, version) {
        Some(path) => Some(format!(
            "{}/{}",
            config::get_package_archive_download_host(),
            &path
        )),
        None => {
            error!("get_package_archive_download_url: failed to get package file path");
            None
        }
    }
}

fn get_package_dir_path<'a>(package_name: &'a str, package_version: &'a str) -> Option<String> {
    let root_dir = config::get_package_root_dir();

    // error!("get_package_dir_path: root_dir: {:?}", root_dir);

    let path = Path::new(&root_dir)
        .join(&package_name)
        .join(&package_version);

    path.to_str().map(|path| path.to_string())
}

#[inline]
fn get_package_file_name<'a>(package_name: &'a str, package_version: &'a str) -> String {
    format!("{}-{}.tar.gz", package_name, package_version)
}

#[inline]
fn get_sha256_file_name<'a>(package_name: &'a str, package_version: &'a str) -> String {
    format!("{}-{}.sha256", package_name, package_version)
}

fn get_package_file_path_from_package_name(package_name: &str) -> Option<String> {
    let connection = &mut server_database::establish_connection();
    connection
        .transaction(|connection| {
            let package_entity = match package_dao::find_by_name(connection, package_name) {
                Ok(entity) => entity,
                Err(e) => {
                    error!("get_package: failed to find package by name: {:?}", e);
                    return Err(e);
                }
            };

            let package_file_path =
                get_package_file_path(package_name, &package_entity.latest_version)
                    .unwrap_or_else(|| String::new());
            Ok(package_file_path)
        })
        .map_or(None, |package_file_path| {
            if package_file_path.is_empty() {
                error!("get_package: failed to get package file path");
                return None;
            }
            Some(package_file_path)
        })
}
