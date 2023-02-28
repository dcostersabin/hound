use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Composer {
    pub packages: Vec<Package>,
}

impl Composer {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("composer")
            .args(&["global", "show"])
            .output()
            .await;

        let length = 1;

        let ecosystem = "composer";

        match output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(package_list) => {
                    let package_list: Vec<&str> = package_list.split("\n").collect();

                    for package_info in package_list {
                        let package: Vec<&str> = package_info.split_whitespace().collect();

                        if package.len() > length {
                            let name = package[0];
                            let version = package[1];
                            let package = Package::new(name, version, ecosystem);
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
