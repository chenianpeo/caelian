use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};
use std::fs;

use std::path::Path;

use reqwest::Client;
use tokio::io::AsyncWriteExt;

use std::process::Command;

/// application list repository
#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    url: String,
    installer: String,
}

#[derive(Serialize, Deserialize)]
struct InstalledPackage {
    name: String,
    version: String,
    installer: String,
    installed_at: String,
}

#[derive(Serialize, Deserialize)]
struct InstalledRepo {
    installed: Vec<InstalledPackage>,
}

#[derive(Deserialize)]
struct PackageRepo {
    packages: Vec<Package>,
}

/// relevant operation command
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
    Install { name: String },

    /// Uninstall a package
    Uninstall { name: String },
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
                println!("{:<10} {:<10} {}", pkg.name, pkg.version, pkg.description);
            }

            println!("--------------------");
            println!("{} packages available", &repo.packages.len());
        }

        Commands::Install { name } => {
            println!("searching package: {name}");

            let repo = load_repo();
            let pkg = find_package(&name, &repo).expect("❌ package not found in repository");

            ensure_download_dir();

            let file_path = format!("downloads/{}_{}.exe", pkg.name, pkg.version);
            println!("⬇ Downloading {} {} ...", pkg.name, pkg.version);

            download_file(&pkg.url, &file_path).await;
            println!("Saved to {}", file_path);
            println!("Running installer...");

            run_installer(&file_path);

            let mut installed_repo = load_installed();

            installed_repo.installed.push(InstalledPackage {
                name: pkg.name.clone(),
                version: pkg.version.clone(),
                installer: file_path.clone(),
                installed_at: chrono::Local::now().to_rfc3339(),
            });
            save_installed(&installed_repo);
            println!("√ Installed successfully");
        }

        Commands::Uninstall { name } => {
            let mut repo = load_installed();

            if let Some(index) = find_installed(&name, &repo) {
                let pkg = &repo.installed[index];

                println!("found installed package: {} {}", pkg.name, pkg.version);
                println!("installer recorded: {}", pkg.installer);

                println!("Running uninstaller...");
                run_uninstaller(&pkg.installer);

                repo.installed.remove(index);
                save_installed(&repo);

                println!("√ Uninstalled successfully");
            } else {
                println!("❌ package not found in installed list");
            }
        }
    }
}

// read app list json from packages.json
fn load_repo() -> PackageRepo {
    let data = fs::read_to_string("packages.json").expect("❌ failed to read packages.json");

    serde_json::from_str::<PackageRepo>(&data).expect("❌ failed to parse packages.json")
}

// create download directory
fn ensure_download_dir() {
    let path = Path::new("downloads");
    if !path.exists() {
        fs::create_dir_all(path).expect("❌ failed to create downloads directory");
    }
}

// download relevant application
// define async function, accept url and file save path
async fn download_file(url: &str, file_path: &str) {
    // create a HTTP client instance
    let client = Client::new();
    // result save to resp variable
    let mut resp = client
        .get(url)
        // send relevant request
        .send()
        // wait async operation complete
        .await
        // error conduct
        .expect("❌ failed to download file");

    // use tokio async file system create file
    let mut file = tokio::fs::File::create(file_path)
        .await
        .expect("❌ failed to create file");

    // read next data chunk from request
    while let Some(chunk) = resp.chunk().await.expect("❌ failed to read chunk") {
        // exit when not data
        file.write_all(&chunk)
            .await
            .expect("❌ failed to write file");
    }
}

// find package in repository
fn find_package<'a>(name: &str, repo: &'a PackageRepo) -> Option<&'a Package> {
    repo.packages.iter().find(|p| p.name == name)
}

// transfer install application
fn run_installer(path: &str) {
    let status = Command::new(path)
        // add command argument
        .arg("/S")
        // conduct command and wait complete, return exit status
        .status()
        // error conduct
        .expect("❌ failed to start installer");

    // exam installer application exit status code
    // exit if success and return true, exit code isn't 0 express install fail
    if !status.success() {
        panic!("❌ installer returned failure");
    }
}

// load installed database
fn load_installed() -> InstalledRepo {
    let data = fs::read_to_string("installed.json")
    .unwrap_or("{\"installed\": []}".to_string());

    serde_json::from_str::<InstalledRepo>(&data)
    .expect("❌ failed to parse installed.json")
}

// save relevant database
fn save_installed(repo: &InstalledRepo) {
    let json = serde_json::to_string_pretty(repo)
    .expect("❌ failed to serialize installed repo");

    fs::write("installed.json", json)
    .expect("❌ failed to write installed.json");
}

// find installed application
fn find_installed(name: &str, repo: &InstalledRepo) -> Option<usize> {
    repo.installed.iter().position(|p| p.name == name)
}

// run uninstall application
fn run_uninstaller(installer: &str) {
    let status = Command::new(installer)
    .arg("/S")
    .arg("/uninstall")
    .status()
    .expect("❌ failed to run uninstaller");

    if !status.success() {
        println!("⚠ uninstaller exited with non-zero status");
    }
}