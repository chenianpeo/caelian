use std::fs;
use std::path::Path;

use crate::models::package::Package;
use crate::models::package_database::PackageDatabase;

pub fn run(keyword: &str) {
    let path = Path::new("data/packages.json");

    if !path.exists() {
        println!("Package database not found: data/packages.json");
        return;
    }

    let data = fs::read_to_string(path).expect("Unable to read packages.json");
    let db: PackageDatabase = serde_json::from_str(&data).expect("JSON parse error");

    let results: Vec<&Package> = db.packages.iter()
    .filter(|p| {
        p.name.to_lowercase().contains(&keyword.to_lowercase())
        || p.description.to_lowercase().contains(&keyword.to_lowercase())
        || p.tags.iter().any(|t| t.to_lowercase().contains(&keyword.to_lowercase()))
    })
    .collect();

    if results.is_empty() {
        println!("No packages found for keyword: {}", keyword);
    } else {
        println!("Found {} package(s):", results.len());
        for pkg in results {
            println!(
                "- {} Version: {} Type: {}, Size: {} MB, URL: {}",
                pkg.name, pkg.version, pkg.installer_type, pkg.size, pkg.url
            );
        }
    }
}
