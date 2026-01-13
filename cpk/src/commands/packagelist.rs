use std::fs;
use std::env;

use crate::models::package::Package;
use crate::models::package_database::PackageDatabase;


pub fn run() {
    let exe_dir = env::current_exe()
    .ok()
    .and_then(|p| p.parent().map(|p| p.to_path_buf()))
    .expect("failed to get executable directory");

    let path = exe_dir.join("data").join("packages.json");

    if !path.exists() {
        println!("packages database not found: data/ packages.json");
        return;
    }

    let data = fs::read_to_string(path)
    .expect("Unable to read packages.json");
    let db: PackageDatabase = serde_json::from_str(&data)
    .expect("JSON parse error");

    println!("===== All Available Packages =====");
    for pkg in db.packages.iter() {
        print_package(pkg);
    }
    println!("===== End of Package List ====");
}

fn print_package(pkg: &Package) {
    println!("Name       : {}", pkg.name);
    println!("Version    : {}", pkg.version);
    println!("Description: {}", pkg.description);
    println!("Type       : {}", pkg.installer_type);
    println!("Size (MB)  : {}", pkg.size);
    // println!("URL        : {}", pkg.url);
    // println!("Silent Args: {}", if pkg.silent_args.is_empty() { "N/A" } else { &pkg.silent_args });
    // println!("Env Path   : {}", pkg.env_path);
    // println!("Checksum   : {}", pkg.checksum);
    // println!("Tags       : {}", pkg.tags.join(", "));
    println!("-------------------------------");
}