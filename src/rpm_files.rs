use crate::Package;
use rayon::prelude::*;
use std::process::Command;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

pub struct RpmFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl RpmFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "rpm";
        self.files.par_iter().for_each(|file| {
            let output = Command::new("rpm").args(&["-qip", file]).output();

            match output {
                Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                    Ok(package_info) => {
                        let lines: Vec<&str> = package_info.split("\n").collect();
                        let mut version = String::new();
                        let mut name = String::new();
                        for line in lines {
                            if line.contains("version") {
                                let version_split: Vec<&str> = line.split(":").collect();
                                version = version_split[1].trim().to_string();
                                continue;
                            }
                            if line.contains("name") {
                                let name_split: Vec<&str> = line.split(":").collect();
                                name = name_split[1].trim().to_string();
                                continue;
                            }
                            if !version.is_empty() & !name.is_empty() {
                                break;
                            }
                        }
                        if !version.is_empty() | !name.is_empty() {
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
