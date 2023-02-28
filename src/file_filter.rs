use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileFilter {
    path: String,
    pub python: Vec<String>,
    pub rust: Vec<String>,
    pub package_json: Vec<String>,
    pub yarn_lock: Vec<String>,
    pub go: Vec<String>,
    pub jar: Vec<String>,
    pub deb: Vec<String>,
    pub rpm: Vec<String>,
    pub apk: Vec<String>,
    pub pom: Vec<String>,
    pub gem: Vec<String>,
    pub composer: Vec<String>,
    pub cs_proj: Vec<String>,
}

impl FileFilter {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            python: Vec::new(),
            rust: Vec::new(),
            package_json: Vec::new(),
            yarn_lock: Vec::new(),
            go: Vec::new(),
            jar: Vec::new(),
            deb: Vec::new(),
            rpm: Vec::new(),
            apk: Vec::new(),
            pom: Vec::new(),
            gem: Vec::new(),
            composer: Vec::new(),
            cs_proj: Vec::new(),
        }
    }
    pub fn start(&mut self) {
        self.run();
    }

    fn run(&mut self) {
        self.get_all_files();
    }

    fn get_all_files(&mut self) {
        for file in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
            let filename = format!("{}", file.path().display().to_string().as_str());

            let is_python = filename.contains("requirements.txt");

            if is_python {
                self.python.push(filename.to_string());
                continue;
            }

            let is_rust = filename.contains("Cargo.toml");

            if is_rust {
                self.rust.push(filename.to_string());
                continue;
            }

            let is_package_json = filename.contains("package.json");

            if is_package_json {
                self.package_json.push(filename.to_string());
                continue;
            }

            let is_yarn_lock = filename.contains("yarn.lock");

            if is_yarn_lock {
                self.yarn_lock.push(filename.to_string());
                continue;
            }

            let is_go_mod = filename.contains("go.mod");

            if is_go_mod {
                self.go.push(filename.to_string());
                continue;
            }

            let is_jar = filename.ends_with(".jar");

            if is_jar {
                self.jar.push(filename.to_string());
                continue;
            }

            let is_deb = filename.ends_with(".deb");

            if is_deb {
                self.deb.push(filename.to_string());
                continue;
            }

            let is_rpm = filename.ends_with(".rpm");

            if is_rpm {
                self.rpm.push(filename.to_string());
                continue;
            }

            let is_apk = filename.ends_with(".apk");

            if is_apk {
                self.apk.push(filename.to_string());
                continue;
            }

            let is_pom = filename.contains("pom.xml");

            if is_pom {
                self.pom.push(filename.to_string());
                continue;
            }

            let is_gem_file = filename.contains("Gemfile");

            if is_gem_file {
                self.gem.push(filename.to_string());
                continue;
            }

            let is_composer_file = filename.contains("composer.json");

            if is_composer_file {
                self.composer.push(filename.to_string());
                continue;
            }

            let is_cs_proj_files = filename.ends_with(".csproj");

            if is_cs_proj_files {
                self.cs_proj.push(filename.to_string());
                continue;
            }
        }
    }
}
