use crate::utils::*;
use crate::Package;
use rayon::prelude::*;
use std::process::Command;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

pub struct Jar {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl Jar {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "jar";
        self.files.par_iter().for_each(|file| {
            let output = Command::new("unzip")
                .args(&["-p", file.as_str(), "META-INF/MANIFEST.MF"])
                .output();

            match output {
                Ok(content) => match from_utf8(&content.stdout) {
                    Ok(manifest) => {
                        let mut version = String::new();
                        let mut name = String::new();
                        if manifest.contains("Bundle-Name:") {
                            name = get_bundle_name(manifest);
                            version = get_bundle_version(manifest);
                        }

                        if manifest.contains("Implementation-Title:") {
                            name = get_implementation_title(manifest);
                            version = get_implementation_version(manifest);
                        }
                        let tmp_packages = shared_package.lock();
                        if !version.is_empty() | !name.is_empty() {
                            match tmp_packages {
                                Ok(mut tmp_packages) => {
                                    let package = Package::new(&name, &version, ecosystem);
                                    tmp_packages.push(package);
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                },
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
