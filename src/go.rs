use crate::Package;
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct Go {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl Go {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let re = Regex::new(r".*.com.*|.*in.*|.*org.*|.*ini.*|.*edu.*").unwrap();
        let ecosystem = "go";
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(&file);
            match content {
                Ok(content) => {
                    let captured = re.find_iter(content.as_str());

                    for i in captured {
                        let package = i.as_str();

                        if package.contains("module ") | package.contains("go ") {
                            continue;
                        } else {
                            match package.trim().split_once(" ") {
                                Some((key, value)) => {
                                    if value.contains("//") {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let _ver: Vec<&str> = value.split("//").collect();
                                                let name = key.trim().to_string();
                                                let version = _ver[0].trim().to_string();
                                                let package =
                                                    Package::new(&name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
                                    } else {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = key.trim().to_string();
                                                let version = value.trim().to_string();
                                                let package =
                                                    Package::new(&name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                                None => {
                                    continue;
                                }
                            }
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
