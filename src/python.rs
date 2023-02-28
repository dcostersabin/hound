use crate::Package;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct Python {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl Python {
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
            let ecosystem = "pip";
            match content {
                Ok(content) => {
                    let collection: Vec<&str> = content.split("\n").collect();

                    for package in collection {
                        let length: usize = 1;

                        let info: Vec<&str> = package.split("==").collect();
                        if &info.len() > &length {
                            let tmp_packages = shared_package.lock();
                            match tmp_packages {
                                Ok(mut tmp_packages) => {
                                    let name = info[0];
                                    let version = info[1];
                                    let package = Package::new(name, version, ecosystem);
                                    tmp_packages.push(package);
                                }
                                Err(_) => {}
                            }
                            continue;
                        }

                        let info: Vec<&str> = package.split(">=").collect();
                        if &info.len() > &length {
                            let tmp_packages = shared_package.lock();
                            match tmp_packages {
                                Ok(mut tmp_packages) => {
                                    let name = info[0];
                                    let version = info[1];
                                    let package = Package::new(name, version, ecosystem);
                                    tmp_packages.push(package);
                                }
                                Err(_) => {}
                            }
                            continue;
                        }

                        let info: Vec<&str> = package.split("<=").collect();
                        if &info.len() > &length {
                            let tmp_packages = shared_package.lock();
                            match tmp_packages {
                                Ok(mut tmp_packages) => {
                                    let name = info[0];
                                    let version = info[1];
                                    let package = Package::new(name, version, ecosystem);
                                    tmp_packages.push(package);
                                }
                                Err(_) => {}
                            }
                            continue;
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
