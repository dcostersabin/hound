extern crate clap;
mod apk;
mod arch;
mod composer;
mod dpkg;
mod gem;
mod npm;
mod package;
mod pip;
mod rpm;
mod system_pipeline;

use apk::Apk;
use arch::Arch;
use clap::{Args, Parser, Subcommand};
use composer::Composer;
use dpkg::Dpkg;
use gem::Gem;
use npm::Npm;
use package::Package;
use pip::Pip;
use rpm::Rpm;
use serde_json::to_string;
use system_pipeline::SystemPipeline;

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
                        println!("{}", path);
                    }
                    None => {
                        println!("Please Provide A Path");
                    }
                }
            }
        }
    }
}
