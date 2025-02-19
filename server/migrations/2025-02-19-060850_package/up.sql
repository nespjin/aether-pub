-- Your SQL goes here
CREATE TABLE package
(
    id                 INTEGER PRIMARY KEY,
    name               TEXT    NOT NULL,
    is_discontinued    BOOLEAN NOT NULL,
    replaced_by        TEXT DEFAULT NULL,
    advisories_updated TEXT DEFAULT NULL,
    latest_version_id  INTEGER NOT NULL,
    created_time       INTEGER NOT NULL,
    updated_time       INTEGER NOT NULL
);

CREATE TABLE package_version
(
    id             INTEGER PRIMARY KEY,
    version        TEXT    NOT NULL,
    retracted      BOOLEAN NOT NULL,
    archive_url    TEXT    NOT NULL,
    archive_sha256 TEXT    NOT NULL,
    pubspec_json   TEXT    NOT NULL,
    package_id     INTEGER NOT NULL,
    created_time   INTEGER NOT NULL,
    updated_time   INTEGER NOT NULL
);