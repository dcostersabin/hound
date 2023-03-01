use crate::Package;
use rayon::prelude::*;
use std::process::Command;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

pub struct ApkFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl ApkFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "apk";
        self.files.par_iter().for_each(|file| {
            let output = Command::new("tar")
                .args(&["xvf", file, ".PKGINFO", "--to-command=cat"])
                .output();

            match output {
                Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                    Ok(pkg_info) => {
                        let mut name = String::new();
                        let mut version = String::new();
                        for line in pkg_info.split("\n").into_iter() {
                            if line.contains("pkgname") {
                                let name_split: Vec<&str> = line.split("=").collect();
                                if name_split.len() > 1 {
                                    name = name_split[1].trim().to_string();
                                    continue;
                                }
                            }

                            if line.contains("pkgver") {
                                let version_split: Vec<&str> = line.split("=").collect();
                                if version_split.len() > 1 {
                                    version = version_split[1].trim().to_string();
                                    continue;
                                }
                            }

                            if !name.is_empty() & !version.is_empty() {
                                break;
                            }
                        }
                        if !name.is_empty() | !version.is_empty() {
                            let tmp_packages = shared_package.lock();
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
