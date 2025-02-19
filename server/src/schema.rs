table! {
    package(name) {
        name -> Text,
        is_discontinued -> Bool,
        replaced_by -> Nullable<Text>,
        advisories_updated -> Nullable<Text>,
        latest_version -> Text,
        created_time -> Integer,
        updated_time -> Integer,
    }
}

table! {
    package_version(version) {
        version -> Text,
        retracted -> Bool,
        archive_url -> Text,
        archive_sha256 -> Text,
        pubspec_json -> Text,
        package_name -> Text,
        created_time -> Integer,
        updated_time -> Integer,
    }
}

joinable!(package_version -> package (package_name));
