use aether_pub_server::service::package_service;
use std::fs::File;
use std::io::Read;

#[test]
fn test_parse_pubspec_from_tar_gz() {
    let file = File::open("tests/service/animation-2.0.11.tar.gz").unwrap();
    let value = package_service::parse_pubspec_from_tar_gz(&file);
    assert_eq!(value.unwrap().get("version").unwrap(), "2.0.11");
}
#[test]
fn test_save_package_and_sha256_file() {
    let file = &mut File::open("tests/service/animation-2.0.11.tar.gz").unwrap();
    let buf = &mut Vec::new();
    file.read_to_end(buf).unwrap();
    let package_name = "animation";
    let package_version = "2.0.11";

    let paths =
        package_service::save_package_and_sha256_file(package_name, package_version, buf);
    println!("{:?}", &paths);

    assert!(paths.is_some());
}
