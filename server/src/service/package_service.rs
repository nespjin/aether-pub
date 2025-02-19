use sha2::{Digest, Sha256};
use std::env;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Read, Write};
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

/// Parses the `pubspec.yaml` file from a `.tar.gz` file and converts it into JSON format.
///
/// This function takes a `File` object representing a `.tar.gz` file as input. It decompresses the file,
/// iterates through its contents, and looks for a file named `pubspec.yaml`. If found, it parses the
/// content of the file into JSON format and returns it.
///
/// # Parameters
/// * `file` - A `File` object representing the `.tar.gz` file to be parsed.
///
/// # Returns
/// Returns an `Option<serde_json::Value>`. If the `pubspec.yaml` file is successfully parsed, its JSON
/// representation is returned. If the file is not found or parsing fails, `None` is returned.
pub fn parse_pubspec_from_tar_gz(file: &File) -> Option<serde_json::Value> {
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let mut pubspec = None;

    let entries_result = archive.entries();
    if let Err(e) = entries_result {
        println!("parse_pubspec_from_tar_gz: failed to get entries: {:?}", e);
        return None;
    }

    for entry_result in entries_result.unwrap() {
        if let Err(e) = entry_result {
            println!("parse_pubspec_from_tar_gz: failed to get entry: {:?}", e);
            continue;
        }

        let mut entry = entry_result.unwrap();
        let entry_path_result = entry.path();

        if let Err(e) = entry_path_result {
            println!(
                "parse_pubspec_from_tar_gz: failed to get entry path: {:?}",
                e
            );
            continue;
        }

        let path = entry_path_result.unwrap();

        if path.ends_with("pubspec.yaml") {
            let mut content = String::new();
            if let Err(e) = entry.read_to_string(&mut content) {
                println!("parse_pubspec_from_tar_gz: failed to read entry: {:?}", e);
                continue;
            }

            // println!("pubspec.yaml content: {}", content);

            // Parse pubspec.yaml to JSON
            let pubspec_json = serde_yaml::from_str::<serde_json::Value>(&content);
            if let Err(_) = pubspec_json {
                return None;
            }
            pubspec = Some(pubspec_json.unwrap());
            break;
        }
    }
    pubspec
}

/// Saves the package file and its corresponding SHA256 checksum file.
///
/// This function takes the package name, version, and content, generates the package file and its SHA256 checksum file,
/// writes the package content to the package file, and writes the SHA256 checksum to the checksum file.
///
/// # Parameters
/// - `package_name`: The name of the package, used to generate the file path.
/// - `package_version`: The version of the package, used to generate the file path.
/// - `buf`: The byte array of the package content, which will be written to the package file.
///
/// # Returns
/// - If successful, returns a tuple containing the paths of the package file and the SHA256 checksum file.
/// - If any step fails, returns `None`.
pub fn save_package_and_sha256_file(
    package_name: &str,
    package_version: &str,
    buf: &[u8],
) -> Option<(String, String)> {
    let package_file_path_opt = get_package_file_path(package_name, package_version);
    if let None = package_file_path_opt {
        println!("save_package_and_sha256_file: failed to get package file path");
        return None;
    }

    let sha256_file_path_opt = get_sha256_file_path(package_name, package_version);
    if let None = sha256_file_path_opt {
        println!("save_package_and_sha256_file: failed to get sha256 file path");
        return None;
    }

    let package_file_path = package_file_path_opt.unwrap();
    let sha256_file_path = sha256_file_path_opt.unwrap();

    // println!(
    //     "save_package_and_sha256_file: package_file_path: {:?}",
    //     &package_file_path
    // );
    // println!(
    //     "save_package_and_sha256_file: sha256_file_path: {:?}",
    //     &sha256_file_path
    // );

    Path::new(&package_file_path).parent().map(|parent| {
        if !parent.exists() {
            create_dir_all(parent).expect("Failed to create package directory");
        }
    });

    Path::new(&sha256_file_path).parent().map(|parent| {
        if !parent.exists() {
            if let Err(_) = create_dir_all(parent) {
                println!("save_package_and_sha256_file: failed to create sha256 directory");
            }
        }
    });

    let package_file_result = File::create(&package_file_path);
    if let Err(e) = package_file_result {
        println!(
            "save_package_and_sha256_file: failed to create package file, {:?}",
            e
        );
        return None;
    }

    let sha256_file_result = File::create(&sha256_file_path);
    if let Err(_) = sha256_file_result {
        println!("save_package_and_sha256_file: failed to create sha256 file");
        return None;
    }

    let mut package_file = package_file_result.unwrap();
    let mut sha256_file = sha256_file_result.unwrap();

    // Calculate the file sha256 hash
    let mut sha256 = Sha256::default();
    sha256.update(buf);
    let hash_code = sha256.finalize();
    let hash_hex = format!("{:x}", hash_code);

    println!("save_package_and_sha256_file: file hash: {:?}", &hash_hex);

    if let (Ok(package_file_size), Ok(sha256_file_size)) = (
        package_file.write(buf),
        sha256_file.write(&hash_hex.as_bytes()),
    ) {
        println!(
            "save_package_and_sha256_file: package_file_size: {:?}, sha256_file_size: {}",
            package_file_size, sha256_file_size
        );
        Some((package_file_path, sha256_file_path))
    } else {
        println!("save_package_and_sha256_file: failed to write file");
        None
    }
}

pub(crate) fn get_package_file_path(package_name: &str, package_version: &str) -> Option<String> {
    get_file_path(package_name, package_version, get_package_file_name)
}

pub(crate) fn get_sha256_file_path(package_name: &str, package_version: &str) -> Option<String> {
    get_file_path(package_name, package_version, get_sha256_file_name)
}

fn get_file_path(
    package_name: &str,
    package_version: &str,
    get_file_name: fn(&str, &str) -> String,
) -> Option<String> {
    let dir_path_opt = get_package_dir_path(package_name, package_version);
    if let None = dir_path_opt {
        println!("get_file_path: failed to get package dir path");
        return None;
    }
    let dir_path = dir_path_opt.unwrap();

    let file_name = get_file_name(package_name, package_version);
    let file_path = Path::new(&dir_path).join(&file_name);

    if let Some(path) = file_path.to_str() {
        return Some(path.to_string());
    }

    println!("get_file_path: failed to get file path");
    None
}

fn get_package_dir_path<'a>(package_name: &'a str, package_version: &'a str) -> Option<String> {
    dotenv::dotenv().ok();
    let root_dir = env::var("PACKAGE_ROOT_DIR").unwrap_or_else(|_| "packages".to_string());

    println!("get_package_dir_path: root_dir: {:?}", root_dir);

    let path = Path::new(&root_dir)
        .join(&package_name)
        .join(&package_version);

    if let Some(path) = path.to_str() {
        return Some(path.to_string());
    }

    println!("get_package_dir_path: failed to get dir path");
    None
}

fn get_package_file_name<'a>(package_name: &'a str, package_version: &'a str) -> String {
    format!("{}-{}.tar.gz", package_name, package_version)
}

fn get_sha256_file_name<'a>(package_name: &'a str, package_version: &'a str) -> String {
    format!("{}-{}.sha256", package_name, package_version)
}
