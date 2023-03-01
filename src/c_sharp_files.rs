use crate::Package;
use rayon::prelude::*;
use roxmltree::Document;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

pub struct CSharpFiles {
    files: Vec<String>,
    pub packages: Vec<Package>,
}

impl CSharpFiles {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files: files,
            packages: Vec::new(),
        }
    }

    pub fn get_packages(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        let ecosystem = ".Net";
        self.files.par_iter().for_each(|file| {
            let content = read_to_string(file);

            match content {
                Ok(content) => {
                    let doc = Document::parse(&content.as_str());

                    match doc {
                        Ok(doc) => {
                            let dependencies = doc
                                .descendants()
                                .find(|n| n.tag_name().name() == "ItemGroup");

                            if !dependencies.is_none() {
                                for i in dependencies.unwrap().children() {
                                    if i.tag_name().name() == "PackageReference" {
                                        let package_name = i.attribute("Include");
                                        let package_version = i.attribute("Version");

                                        if package_name.is_some() & package_version.is_some() {
                                            let tmp_packages = shared_package.lock();
                                            match tmp_packages {
                                                Ok(mut tmp_packages) => {
                                                    let name = package_name.unwrap().to_string();
                                                    let version =
                                                        package_version.unwrap().to_string();
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
