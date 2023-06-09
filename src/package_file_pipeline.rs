use crate::ApkFiles;
use crate::CSharpFiles;
use crate::ComposerFiles;
use crate::DpkgFiles;
use crate::FileFilter;
use crate::GemFiles;
use crate::Go;
use crate::Jar;
use crate::Package;
use crate::PkgJson;
use crate::PomFiles;
use crate::Python;
use crate::RpmFiles;
use crate::Rust;
use crate::Yarn;

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
        self.yarn();
        self.go();
        self.jar();
        self.dpkg();
        self.rpm();
        self.apk();
        self.pom();
        self.gem();
        self.composer();
        self.csharp();
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

    fn yarn(&mut self) {
        let mut yarn_obj = Yarn::new(self.file_filter.yarn_lock.clone());
        yarn_obj.get_packages();
        self.packages.extend(yarn_obj.packages);
    }

    fn go(&mut self) {
        let mut go_obj = Go::new(self.file_filter.go.clone());
        go_obj.get_packages();
        self.packages.extend(go_obj.packages);
    }

    fn jar(&mut self) {
        let mut jar_obj = Jar::new(self.file_filter.jar.clone());
        jar_obj.get_packages();
        self.packages.extend(jar_obj.packages);
    }

    fn dpkg(&mut self) {
        let mut dpkg_obj = DpkgFiles::new(self.file_filter.deb.clone());
        dpkg_obj.get_packages();
        self.packages.extend(dpkg_obj.packages);
    }

    fn rpm(&mut self) {
        let mut rpm_obj = RpmFiles::new(self.file_filter.rpm.clone());
        rpm_obj.get_packages();
        self.packages.extend(rpm_obj.packages);
    }

    fn apk(&mut self) {
        let mut apk_obj = ApkFiles::new(self.file_filter.apk.clone());
        apk_obj.get_packages();
        self.packages.extend(apk_obj.packages);
    }

    fn pom(&mut self) {
        let mut pom_obj = PomFiles::new(self.file_filter.pom.clone());
        pom_obj.get_packages();
        self.packages.extend(pom_obj.packages);
    }

    fn gem(&mut self) {
        let mut gem_obj = GemFiles::new(self.file_filter.gem.clone());
        gem_obj.get_packages();
        self.packages.extend(gem_obj.packages);
    }

    fn composer(&mut self) {
        let mut composer_obj = ComposerFiles::new(self.file_filter.composer.clone());
        composer_obj.get_packages();
        self.packages.extend(composer_obj.packages);
    }

    fn csharp(&mut self) {
        let mut csharp_obj = CSharpFiles::new(self.file_filter.cs_proj.clone());
        csharp_obj.get_packages();
        self.packages.extend(csharp_obj.packages);
    }
}
