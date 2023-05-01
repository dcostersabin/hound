# Hound

## Getting started

Hound is indeed inspired by [Syft](https://github.com/anchore/syft), another open-source tool for finding SBOMs, 

Hound is an open-source tool written in Rust that is designed to help users find Software Bill of Materials (SBOM) for a given project. SBOMs are an inventory of the components and dependencies used in software applications, and are an important tool for managing security risks, license compliance, and vulnerability management.

Hound uses a combination of static analysis and dependency resolution to build a complete picture of the components used in a project. It supports a variety of package managers and build systems, including Cargo, npm, and pip, and can be integrated with other tools in a DevOps pipeline.

![Hound](https://github.com/dcostersabin/hound/blob/develop/assets/hound.png)

## Features
- Generates SBOMs for container images, filesystems, archives, and more to discover packages and libraries

## Supported Ecosystems

- Alpine (apk,binaries)
- Debian (dpkg,binaries)
- Arch (pacman)
- Dotnet (deps.json)
- C# (cocoapods)
- Go (go.mod, Go binaries)
- Java (jar,pom)
- JavaScript (npm, yarn)
- PHP (composer,binaries)
- Python (requirements.txt,binaries)
- Red Hat (rpm,binaries)
- Ruby (gem,binaries)
- Rust (cargo.toml,cargo.lock,binariesk)

## Compiling Binary

Currently Hound requires users to compile the tool themselves, as precompiled binaries are not yet available. However, this may change in future iterations of the tool, as the development team continues to refine and enhance its capabilities.

To Compile Use The Following Code:

``` cargo build --release ```

After you have compiled You can find the binary at target/release

### Commands

#### For System Dependencies

``` ./hound detect -s  ```

#### For File systems

``` ./hound detect <PATH> ```


## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

