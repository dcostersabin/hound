use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Dpkg {
    pub packages: Vec<Package>,
}

impl Dpkg {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("dpkg-query")
            .args(&["--showformat=${Package}==${Version}\n", "--show"])
            .output()
            .await;

        match output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(packages) => {
                    let package_list: Vec<&str> = packages.split("\n").collect();
                    for package in package_list {
                        let length = 1;
                        let info: Vec<&str> = package.split("==").collect();
                        if info.len() > length {
                            let name = info[0].to_string();
                            let version = info[1].to_string();
                            let ecosystem = "dpkg";
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
