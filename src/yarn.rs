use crate::Package;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use yarn_lock_parser::parse_str;

pub struct Yarn {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl Yarn {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "yarn";
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(&file);

            match content {
                Ok(content) => {
                    let entries = parse_str(content.as_str());

                    match entries {
                        Ok(entries) => {
                            for entry in entries {
                                for i in entry.dependencies {
                                    let tmp_packages = shared_package.lock();
                                    match tmp_packages {
                                        Ok(mut tmp_packages) => {
                                            let name = i.0;
                                            let version = i.1;
                                            let package = Package::new(&name, &version, ecosystem);
                                            tmp_packages.push(package);
                                        }
                                        Err(_) => continue,
                                    }
                                }
                            }
                        }
                        Err(_) => {}
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
