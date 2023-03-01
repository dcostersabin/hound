extern crate clap;
mod apk;
mod arch;
mod composer;
mod dpkg;
mod dpkg_files;
mod file_filter;
mod gem;
mod go;
mod jar;
mod npm;
mod package;
mod package_file_pipeline;
mod pip;
mod pkg_json;
mod python;
mod rpm;
mod rpm_files;
mod rust;
mod system_pipeline;
mod utils;
mod yarn;

use apk::Apk;
use arch::Arch;
use clap::{Args, Parser, Subcommand};
use composer::Composer;
use dpkg::Dpkg;
use dpkg_files::DpkgFiles;
use file_filter::FileFilter;
use gem::Gem;
use go::Go;
use jar::Jar;
use npm::Npm;
use package::Package;
use package_file_pipeline::PackageFilePipeline;
use pip::Pip;
use pkg_json::PkgJson;
use python::Python;
use rpm::Rpm;
use rpm_files::RpmFiles;
use rust::Rust;
use serde_json::to_string;
use system_pipeline::SystemPipeline;
use yarn::Yarn;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    system: bool,
}

#[derive(Subcommand)]
enum Commands {
    Detect(DetectArgs),
}

#[derive(Args)]
struct DetectArgs {
    path: Option<String>,
}

fn print_json(data: &Vec<Package>) {
    let json_data = to_string(data);
    match json_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Detect(path) => {
            if cli.system {
                let mut sys_obj = SystemPipeline::new();
                sys_obj.start().await;
                print_json(&sys_obj.packages);
            } else {
                match path.path.as_ref() {
                    Some(path) => {
                        let mut package_file_obj = PackageFilePipeline::new(path);
                        package_file_obj.start();
                        print_json(&package_file_obj.packages);
                    }
                    None => {
                        println!("Please Provide A Path");
                    }
                }
            }
        }
    }
}
