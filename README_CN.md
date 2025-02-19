# Aether Pub

[English](README.md)

Aether Pub 是一个用于在自主机服务器上托管您的私有dart包的框架，它包含 Server 端（使用 Rocket.rs 编写）和 Web 端 （使用 React 编写） 两个部分。

## 快速开始

1. 克隆存储库。

```shell
git clone git@github.com:nespjin/aether-pub.git  ~/
```

2. 运行服务器

```shell
cd ~/ aether-pub /server/
cargo run
```

3. 尝试上传和下载 dart 包

```shell
cd ~/ aether-pub /examples/upload_package_example
flutter pub publish
```

现在可以在 ~/aether-pub/server/packages/upload_package_example/ 目录中找到发布的包。

```shell
cd ~/ aether-pub/examples/download_package_example
flutter pub get
```

现在可以在 download_package_example 中的 pubspec.lock 中找到从服务器获取的 upload_package_example 包的元数据。
