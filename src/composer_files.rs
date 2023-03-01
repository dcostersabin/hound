use crate::Package;
use json::parse;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct ComposerFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl ComposerFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "composer";
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(file);

            match content {
                Ok(content) => {
                    let parsed = parse(content.as_str());
                    match parsed {
                        Ok(json) => {
                            if json.has_key("require") {
                                let dependencies = &json["require"];
                                for (key, value) in dependencies.entries() {
                                    if !value.to_string().is_empty() {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = key.trim();
                                                let version = value.to_string();
                                                let package =
                                                    Package::new(name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
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
