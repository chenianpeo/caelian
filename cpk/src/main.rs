use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    url: String,
    installer: String,
}

#[derive(Deserialize)]
struct PackageRepo {
    packages: Vec<Package>,
}

#[derive(Parser)]
#[command(name = "cpk")]
#[command(about = "Rust Package Keeper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available packages
    List,

    /// Install a package
    Install {
        name: String,
    },

    /// Uninstall a package
    Uninstall {
        name: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let repo = load_repo();

            println!("Available packages:");
            println!("--------------------");

            for pkg in &repo.packages {
                println!(
                    "{:<10} {:<10} {}",
                    pkg.name,
                    pkg.version,
                    pkg.description
                );
            }

            println!("--------------------");
            println!("{} packages available", &repo.packages.len());
        }

        Commands::Install { name } => {
            println!("Install package: {name}");
        }

        Commands::Uninstall { name } => {
            println!("Uninstall packages: {name}");
        }
    }
}

fn load_repo() -> PackageRepo {
    let data = fs::read_to_string("packages.json")
    .expect("❌ failed to read packages.json");

    serde_json::from_str::<PackageRepo>(&data)
    .expect("❌ failed to parse packages.json")
}