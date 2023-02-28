use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Pip {
    pub packages: Vec<Package>,
}

impl Pip {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self, pip_3: bool) {
        let mut ecosystem = "pip";
        let mut output = Command::new("pip").args(&["list"]).output().await;
        if pip_3 {
            output = Command::new("pip").args(&["list"]).output().await;
            ecosystem = "pip3";
        }

        match output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(packages) => {
                    let info: Vec<&str> = packages.split("\n").collect();

                    for i in info {
                        if i.contains("package") | i.contains("version") | i.contains("---") {
                            continue;
                        }
                        let length = 1;
                        let package: Vec<&str> = i.split_whitespace().collect();
                        if package.len() > length {
                            let name = package[0].trim().to_string();
                            let version = package[1].trim().to_string();
                            let package = Package::new(&name, &version, ecosystem);
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
