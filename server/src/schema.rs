table! {
    package {
        id -> Integer,
        name -> Text,
        is_discontinued -> Bool,
        replaced_by -> Nullable<Text>,
        advisories_updated -> Nullable<Text>,
        latest_version_id -> Integer,
        created_time -> Integer,
        updated_time -> Integer,
    }
}

table! {
    package_version {
        id -> Integer,
        version -> Text,
        retracted -> Bool,
        archive_url -> Text,
        archive_sha256 -> Text,
        pubspec_json -> Text,
        package_id -> Integer,
        created_time -> Integer,
        updated_time -> Integer,
    }
}

joinable!(package_version -> package (package_id));
