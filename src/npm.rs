use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Npm {
    pub packages: Vec<Package>,
}

impl Npm {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("npm").args(&["list", "-g"]).output().await;
        let ecosystem = "npm";

        let length = 1;
        match output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(packages) => {
                    let package_list: Vec<&str> = packages.split("\n").collect();

                    for package_info in package_list {
                        if package_info.contains("versions") {
                            continue;
                        }
                        let package_clean: Vec<&str> = package_info.split_whitespace().collect();

                        if package_clean.len() > length {
                            let package: Vec<&str> = package_clean[1].split("@").collect();
                            if package.len() > length {
                                let name = package[0].to_string();
                                let version = package[1].to_string();
                                let package = Package::new(&name, &version, ecosystem);
                                self.packages.push(package);
                            }
                        }
                    }
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}
