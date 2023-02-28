use crate::Package;
use regex::Regex;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Apk {
    pub packages: Vec<Package>,
}

impl Apk {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("apk").args(&["info", "-v"]).output().await;

        let version_re = Regex::new("-[0-9].*").unwrap();
        let ecosystem = "apk";

        match output {
            Ok(content) => match from_utf8(&&content.stdout.to_ascii_lowercase()) {
                Ok(package_list) => {
                    let packages: Vec<&str> = package_list.split("\n").collect();

                    for package_info in packages {
                        let package_name: Vec<&str> = version_re.split(package_info).collect();
                        if package_name.len() > 1 {
                            let name = package_name[0];
                            let version = package_info
                                .replace(package_name[0], "")
                                .replacen("-", "", 1);
                            let package = Package::new(name, &version, ecosystem);
                            self.packages.push(package);
                        }
                    }
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}
