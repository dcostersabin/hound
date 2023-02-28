use crate::FileFilter;
use crate::Package;
use crate::PkgJson;
use crate::Python;
use crate::Rust;

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
        self.rust();
        self.pkg_json();
    }

    fn python(&mut self) {
        let mut py_obj = Python::new(self.file_filter.python.clone());
        py_obj.get_packages();
        self.packages.extend(py_obj.packages);
    }

    fn rust(&mut self) {
        let mut rust_obj = Rust::new(self.file_filter.rust.clone());
        rust_obj.get_packages();
        self.packages.extend(rust_obj.packages);
    }

    fn pkg_json(&mut self) {
        let mut pkg_json_obj = PkgJson::new(self.file_filter.package_json.clone());
        pkg_json_obj.get_packages();
        self.packages.extend(pkg_json_obj.packages);
    }
}
