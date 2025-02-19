-- Your SQL goes here
CREATE TABLE package
(
    name               TEXT PRIMARY KEY NOT NULL,
    is_discontinued    BOOLEAN          NOT NULL,
    replaced_by        TEXT DEFAULT NULL,
    advisories_updated TEXT DEFAULT NULL,
    latest_version     TEXT             NOT NULL,
    created_time       INTEGER          NOT NULL,
    updated_time       INTEGER          NOT NULL
);

CREATE TABLE package_version
(
    version        TEXT PRIMARY KEY NOT NULL,
    retracted      BOOLEAN          NOT NULL,
    archive_url    TEXT             NOT NULL,
    archive_sha256 TEXT             NOT NULL,
    pubspec_json   TEXT             NOT NULL,
    package_name   TEXT             NOT NULL,
    created_time   INTEGER          NOT NULL,
    updated_time   INTEGER          NOT NULL
);