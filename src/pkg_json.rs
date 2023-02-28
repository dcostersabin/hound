use crate::Package;
use npm_package_json::Package as JsonPackage;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct PkgJson {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl PkgJson {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "npm";
        self.files.par_iter().for_each(|file| {
            let packages = JsonPackage::from_path(file);

            match packages {
                Ok(packages) => {
                    let dependencies = packages.dependencies;
                    let dev_dependencies = packages.dev_dependencies;
                    let bundle_dependencies = packages.bundled_dependencies;
                    let optional_dependencies = packages.optional_dependencies;

                    for i in dependencies {
                        let tmp_packages = shared_package.lock();
                        match tmp_packages {
                            Ok(mut tmp_packages) => {
                                let name = i.0;
                                let version = i.1;
                                let package = Package::new(&name, &version, ecosystem);
                                tmp_packages.push(package);
                            }
                            Err(_) => {}
                        }
                    }
                    for i in dev_dependencies {
                        let tmp_packages = shared_package.lock();
                        match tmp_packages {
                            Ok(mut tmp_packages) => {
                                let name = i.0;
                                let version = i.1;
                                let package = Package::new(&name, &version, ecosystem);
                                tmp_packages.push(package);
                            }
                            Err(_) => {}
                        }
                    }
                    for i in bundle_dependencies {
                        let tmp_packages = shared_package.lock();
                        match tmp_packages {
                            Ok(mut tmp_packages) => {
                                let name = i.0;
                                let version = i.1;
                                let package = Package::new(&name, &version, ecosystem);
                                tmp_packages.push(package);
                            }
                            Err(_) => {}
                        }
                    }
                    for i in optional_dependencies {
                        let tmp_packages = shared_package.lock();
                        match tmp_packages {
                            Ok(mut tmp_packages) => {
                                let name = i.0;
                                let version = i.1;
                                let package = Package::new(&name, &version, ecosystem);
                                tmp_packages.push(package);
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }
        });

        let tmp_packages = shared_package.lock();
        match tmp_packages {
            Ok(tmp_packages) => {
                self.packages.extend(tmp_packages.to_vec());
            }
            Err(_) => {}
        }
    }
}
