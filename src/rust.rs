use crate::Package;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use taplo::parser::parse;

pub struct Rust {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl Rust {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(file);
            let ecosystem = "cargo";
            match content {
                Ok(content) => {
                    let parse_result = parse(content.as_str()).into_dom();
                    let dependencies = parse_result.get("dependencies").to_string();
                    let packages = dependencies.split("\n");

                    for i in packages {
                        let package: Vec<&str> = i.split("\n").collect();
                        for package_string in package {
                            match package_string.split_once("=") {
                                Some((key, value)) => {
                                    let length = 1;
                                    let version: Vec<&str> = value.split(",").collect();
                                    if version.len() > length {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = key.trim();
                                                let version = version[0]
                                                    .trim()
                                                    .replace('"', "")
                                                    .replace("{", "")
                                                    .replace("version", "")
                                                    .replace("=", "")
                                                    .replace("\\", "")
                                                    .replace("\"", "")
                                                    .replace(" ", "");
                                                let package =
                                                    Package::new(name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
                                    } else {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = key.trim();
                                                let version = value
                                                    .trim()
                                                    .replace('"', "")
                                                    .replace("{", "")
                                                    .replace("version", "")
                                                    .replace("=", "")
                                                    .replace("\\", "")
                                                    .replace("\"", "")
                                                    .replace(" ", "");
                                                let package =
                                                    Package::new(name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                                None => {
                                    continue;
                                }
                            };
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
