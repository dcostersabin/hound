use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Rpm {
    pub packages: Vec<Package>,
}

impl Rpm {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("rpm")
            .args(&["-qa", "--queryformat", " %{NAME}:%{VERSION}\n"])
            .output()
            .await;

        let ecosystem = "rpm";

        match output {
            Ok(output) => match from_utf8(&output.stdout.to_ascii_lowercase()) {
                Ok(content) => {
                    let package_list = content.split("\n").into_iter();
                    for package in package_list {
                        let package_info: Vec<&str> = package.split(":").collect();
                        if package_info.len() > 1 {
                            let name = package_info[0];
                            let version = package_info[1];
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
