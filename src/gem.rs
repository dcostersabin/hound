use crate::Package;
use std::str::from_utf8;
use tokio::process::Command;

pub struct Gem {
    pub packages: Vec<Package>,
}

impl Gem {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub async fn get_installed(&mut self) {
        let output = Command::new("gem").args(&["list"]).output().await;
        let ecosystem = "gem";

        match output {
            Ok(output) => match from_utf8(&output.stdout.to_ascii_lowercase()) {
                Ok(content) => {
                    let lines = content.split("\n").into_iter();

                    for line in lines {
                        match line.trim().split_once(" ") {
                            Some((package_name, package_version)) => {
                                let name = package_name.trim();
                                let version = package_version
                                    .replace(":", "")
                                    .replace(")", "")
                                    .replace("(", "")
                                    .replace("default", "");
                                if !name.is_empty() & !version.is_empty() {
                                    let package = Package::new(&name, &version, ecosystem);
                                    self.packages.push(package);
                                }
                            }
                            None => {
                                continue;
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
