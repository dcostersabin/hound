use crate::FileFilter;
use crate::Package;
use crate::Python;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct PackageFilePipeline {
    file_filter: FileFilter,
    pub packages: Vec<Package>,
}

impl PackageFilePipeline {
    pub fn new(path: &str) -> Self {
        Self {
            file_filter: FileFilter::new(path),
            packages: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.run();
    }

    fn run(&mut self) {
        self.filter_file();
        self.start_search();
    }

    fn filter_file(&mut self) {
        self.file_filter.start();
    }
    fn start_search(&mut self) {
        self.python();
    }

    fn python(&mut self) {
        let packages: Vec<Package> = Vec::new();
        let shared_package = Arc::new(Mutex::new(packages));
        self.file_filter.python.par_iter().for_each(|file| {
            let mut py_obj = Python::new(file);
            py_obj.get_packages();
            let tmp_packages = shared_package.lock();
            match tmp_packages {
                Ok(mut tmp_packages) => {
                    for package in py_obj.packages {
                        tmp_packages.push(package);
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
