use crate::Package;
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct GemFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl GemFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "gem";
        self.files.par_iter().for_each(|file| {
            if file.to_lowercase().contains("gemfile.lock") {
                return;
            }
            let content = read_to_string(&file);

            let version_re = Regex::new(r"'[~>|>=|<=].*").unwrap();

            match content {
                Ok(content) => {
                    let lines = content.split("\n");

                    for line in lines {
                        if line.contains("#") {
                            continue;
                        }
                        let mut name = String::new();
                        let mut version = String::new();
                        if line.contains("gem") && !line.contains("source") {
                            let filter_line = line.replace("gem ", "");
                            let name_line = filter_line.replace("'", "").replace('"', "");
                            let line_split: Vec<&str> = name_line.split(",").collect();
                            if line_split.len() > 1 {
                                name = line_split[0].trim().to_string();

                                let re_match = version_re.find(&line);

                                if !re_match.is_some() {
                                    version = "N/A".to_string();
                                } else {
                                    version = re_match
                                        .unwrap()
                                        .as_str()
                                        .replace("'", "")
                                        .replace("require:", "")
                                        .replace("false", "")
                                        .trim()
                                        .to_string();
                                }
                            } else {
                                name = name_line.trim().to_string();
                                version = "N/A".to_string();
                            }

                            if !name.is_empty() & !version.is_empty() {
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
