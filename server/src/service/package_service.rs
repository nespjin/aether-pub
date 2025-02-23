use crate::config;
use crate::database::package_entity::PackageEntity;
use crate::database::package_version_entity::PackageVersionEntity;
use crate::database::{package_dao, package_version_dao, sqlite_database};
use sha2::{Digest, Sha256};
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

pub fn get_package_details() {

}

pub fn get_package_readme() {}

pub fn get_package_changelog() {}

pub fn get_package_example(){}

pub fn get_package_installing(){}

pub fn get_package_versions() {}

pub fn query_packages(keyword: &str, page: u32) {}

pub fn save_new_package_version_with_tar_file(
    package_tmp_file_path: &String,
) -> Option<(String, String)> {
    let Ok(mut package_tmp_file) = File::open(package_tmp_file_path) else {
        println!("save_package_and_sha256_file: failed to open package tmp file");
        return None;
    };

    let Some(pubspec) = parse_pubspec_from_tar_gz(&package_tmp_file) else {
        println!("save_package_and_sha256_file: failed to parse pubspec.yaml");
        return None;
    };

    // Reset file pointer to the beginning for subsequent reads
    if let Err(_) = package_tmp_file.seek(std::io::SeekFrom::Start(0)) {
        println!("save_package_and_sha256_file: failed to reset file pointer");
        return None;
    }

    let (Some(package_name), Some(package_version)) =
        (pubspec["name"].as_str(), pubspec["version"].as_str())
    else {
        println!("save_package_and_sha256_file: failed to extract package name and version");
        return None;
    };

    let Some(package_file_path) = get_package_file_path(package_name, package_version) else {
        println!("save_package_and_sha256_file: failed to get package file path");
        return None;
    };

    let Some(sha256_file_path) = get_sha256_file_path(package_name, package_version) else {
        println!("save_package_and_sha256_file: failed to get sha256 file path");
        return None;
    };

    // println!(
    //     "save_package_and_sha256_file: package_file_path: {:?}",
    //     &package_file_path
    // );
    // println!(
    //     "save_package_and_sha256_file: sha256_file_path: {:?}",
    //     &sha256_file_path
    // );

    // Ensure the parent directories for the package and SHA256 files exist
    Path::new(&package_file_path).parent().map(|parent| {
        if !parent.exists() {
            if let Err(_) = create_dir_all(parent) {
                println!("save_package_and_sha256_file: failed to create package directory");
            }
        }
    });

    Path::new(&sha256_file_path).parent().map(|parent| {
        if !parent.exists() {
            if let Err(_) = create_dir_all(parent) {
                println!("save_package_and_sha256_file: failed to create sha256 directory");
            }
        }
    });

    // Create the package file and SHA256 file
    let Ok(mut package_file) = File::create(&package_file_path) else {
        println!("save_package_and_sha256_file: failed to create package file");
        return None;
    };

    let Ok(mut sha256_file) = File::create(&sha256_file_path) else {
        remove_package_and_sha256_file(&package_file_path, "");
        println!("save_package_and_sha256_file: failed to create sha256 file");
        return None;
    };

    let mut sha256 = Sha256::default();
    let mut package_tmp_file_reader = BufReader::new(&package_tmp_file);

    let mut buffer = [0; 1024];
    let mut package_file_size = 0;

    // Read the temporary package file, calculate its SHA256 hash, and write to the package file
    loop {
        if let Ok(bytes_read) = package_tmp_file_reader.read(&mut buffer) {
            println!("save_package_and_sha256_file: bytes_read: {:?}", bytes_read);
            if bytes_read == 0 {
                break;
            }
            let _ = &sha256.update(&buffer[..bytes_read]);
            package_file_size += bytes_read;

            if let Err(_) = package_file.write(&buffer[..bytes_read]) {
                remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
                println!("save_package_and_sha256_file: failed to write package file");
                return None;
            }
        } else {
            remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
            println!("save_package_and_sha256_file: failed to read tmp package file");
            return None;
        }
    }

    if package_file_size == 0 {
        remove_package_and_sha256_file(&package_file_path, &sha256_file_path);
        println!("save_package_and_sha256_file: package file size is 0");
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

    println!("save_package_and_sha256_file: file hash: {:?}", &hash_hex);

    // Write the SHA256 hash to the hash file and update the database
    if let Ok(sha256_file_size) = sha256_file.write(&hash_hex.as_bytes()) {
        println!(
            "save_package_and_sha256_file: package_file_size: {:?}, sha256_file_size: {}",
            package_file_size, sha256_file_size
        );
        update_database(package_version, &pubspec);
        Some((package_file_path, sha256_file_path))
    } else {
        println!("save_package_and_sha256_file: failed to write file");
        None
    }
}
pub fn parse_pubspec_from_tar_gz(file: &File) -> Option<serde_json::Value> {
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let mut pubspec = None;

    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(e) => {
            println!("parse_pubspec_from_tar_gz: failed to get entries: {:?}", e);
            return None;
        }
    };

    for entry_result in entries {
        let mut entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                println!("parse_pubspec_from_tar_gz: failed to get entry: {:?}", e);
                continue;
            }
        };

        let entry_path = match entry.path() {
            Ok(path) => path,
            Err(e) => {
                println!(
                    "parse_pubspec_from_tar_gz: failed to get entry path: {:?}",
                    e
                );
                continue;
            }
        };

        if entry_path.ends_with("pubspec.yaml") {
            let mut content = String::new();
            if let Err(e) = entry.read_to_string(&mut content) {
                println!("parse_pubspec_from_tar_gz: failed to read entry: {:?}", e);
                return None;
            }

            // println!("pubspec.yaml content: {}", content);

            // Parse pubspec.yaml to JSON
            pubspec = match serde_yaml::from_str::<serde_json::Value>(&content) {
                Ok(json) => Some(json),
                Err(e) => {
                    println!(
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

fn remove_package_and_sha256_file(package_file_path: &str, sha256_file_path: &str) {
    if !package_file_path.is_empty() {
        if let Err(_) = remove_file(package_file_path) {
            println!("remove_package_and_sha256_file: failed to remove package file");
        }
    }

    if !sha256_file_path.is_empty() {
        if let Err(_) = remove_file(sha256_file_path) {
            println!("remove_package_and_sha256_file: failed to remove sha256 file");
        }
    }
}

fn update_database(package_version: &str, pubspec: &serde_json::Value) {
    let connection = &mut sqlite_database::establish_connection();

    // .unwrap();
    if let Err(e) = package_version_dao::upsert(
        connection,
        &PackageVersionEntity::new_with_pubspec(&pubspec, false),
    ) {
        println!("update_database: failed to upsert package version: {:?}", e);
        return;
    }

    if let Err(e) = package_dao::upsert(
        connection,
        &PackageEntity::new_with_pubspec(&pubspec, package_version),
    ) {
        println!("update_database: failed to upsert package: {:?}", e);
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
        println!("get_file_path: failed to get package dir path");
        return None;
    };

    let file_name = get_file_name(package_name, package_version);
    let file_path = Path::new(&dir_path).join(&file_name);

    file_path.to_str().map(|path| path.to_string())
}

fn get_package_dir_path<'a>(package_name: &'a str, package_version: &'a str) -> Option<String> {
    dotenv::dotenv().ok();
    let root_dir = config::get_package_root_dir();

    println!("get_package_dir_path: root_dir: {:?}", root_dir);

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
