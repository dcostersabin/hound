use crate::Package;
use std::fs::read_to_string;

pub struct Python {
    file: String,
    pub packages: Vec<Package>,
}

impl Python {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let content = read_to_string(&self.file);
        let ecosystem = "pip";
        match content {
            Ok(content) => {
                let collection: Vec<&str> = content.split("\n").collect();

                for package in collection {
                    let length: usize = 1;
                    let info: Vec<&str> = package.split("==").collect();
                    if &info.len() > &length {
                        let name = info[0];
                        let version = info[1];
                        let package = Package::new(name, version, ecosystem);
                        self.packages.push(package);
                        continue;
                    }

                    let info: Vec<&str> = package.split(">=").collect();
                    if &info.len() > &length {
                        let name = info[0];
                        let version = info[1];
                        let package = Package::new(name, version, ecosystem);
                        self.packages.push(package);
                    }

                    let info: Vec<&str> = package.split("<=").collect();
                    if &info.len() > &length {
                        let name = info[0];
                        let version = info[1];
                        let package = Package::new(name, version, ecosystem);
                        self.packages.push(package);
                    }
                }
            }
            Err(_) => {}
        }
    }
}
