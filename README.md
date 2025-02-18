# Aether Pub

[中文](README_CN.md)

Aether Pub is a framework for hosting your private dart package on a self-hosting Server, consisting of both the server
side (written in Rocket.rs) and the Web side (written in React).

## Quick Start

1. Setup environment

- Install Rust and Cargo : Checkout https://www.rust-lang.org/tools/install
- Install Flutter or Dart : Checkout https://docs.flutter.dev/get-started/install or https://dart.dev/get-dart

2. Clone the repository.

```shell
git clone git@github.com:nespjin/aether-pub.git  ~/
```

3. Run the server

```shell
cd ~/ aether-pub/server/
cargo run
```

4. Try uploading and downloading the dart package

```shell
cd ~/ aether-pub/examples/upload_package_example
flutter pub publish
```

Can find released package in `~/aether-pub/server/packages/upload_package_example` now.

```shell
cd ~/ aether-pub/examples/download_package_example
flutter pub get
```

The metadata for package `upload_package_example` fetched from the server can now be found in pubspec.lock in
download_package_example.
