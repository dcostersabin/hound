use crate::utils::get_children;
use crate::Package;
use rayon::prelude::*;
use roxmltree::Document;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct PomFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl PomFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = "maven";
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(file);

            match content {
                Ok(content) => {
                    let doc = Document::parse(&content.as_str());
                    match doc {
                        Ok(doc) => {
                            let dependencies = doc
                                .descendants()
                                .find(|n| n.tag_name().name() == "dependencies");
                            let plugins =
                                doc.descendants().find(|n| n.tag_name().name() == "plugins");

                            if !dependencies.is_none() {
                                for i in dependencies.unwrap().children() {
                                    let package: (String, String) = get_children(i);

                                    if !package.0.is_empty() & !package.1.is_empty() {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = package.0;
                                                let version = package.1;
                                                let package =
                                                    Package::new(&name, &version, ecosystem);
                                                tmp_packages.push(package);
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                            }

                            if !plugins.is_none() {
                                for i in plugins.unwrap().children() {
                                    let package: (String, String) = get_children(i);

                                    if !package.0.is_empty() & !package.1.is_empty() {
                                        let tmp_packages = shared_package.lock();
                                        match tmp_packages {
                                            Ok(mut tmp_packages) => {
                                                let name = package.0;
                                                let version = package.1;
                                                let package =
                                                    Package::new(&name, &version, ecosystem);
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
