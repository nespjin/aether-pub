#!/bin/bash
CURRENT_PATH=$(dirname $(readlink -f "$0"))
cd $CURRENT_PATH

set -e

## configure these for your environment
PKG="aether-pub-server"                             # cargo package name
TARGET="x86_64-unknown-linux-gnu"                   # remote target
#TARGET="x86_64-pc-windows-gnu"                      # remote target
ASSETS=(".env" "Rocket.toml" "static" "templates")  # list of assets to bundle
BUILD_DIR="target/${TARGET}/release"                # cargo build directory
DIST_DIR="target/${TARGET}/dist"

if [ ! -d "$DIST_DIR" ]; then
    mkdir -p "$DIST_DIR"
fi

## ensure target toolchain is present
rustup target add $TARGET

## cross-compile
cargo zigbuild --target $TARGET --release

## bundle
tar -cvzf "${DIST_DIR}/${PKG}.tar.gz" "${ASSETS[@]}" -C "${BUILD_DIR}" "${PKG}"