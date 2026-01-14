use std::fs;

use crate::{
    core::{download, installer},
    db::{installed, manifest},
    utils::fs as utils_fs,
};

pub fn run(package: &str) {
    println!("cpk install called with package: {package}");

    let exe_dir = utils_fs::get_exe_dir().expect("cannot get exe dir");

    let pkg = match manifest::find_package(package) {
        Some(p) => p,
        None => {
            eprintln!("package {} not found in package.json", package);
            return;
        }
    };

    let packages_dir = exe_dir.join("packages");
    let apps_dir = exe_dir.join("apps");

    utils_fs::create_dir_all(&packages_dir).unwrap();
    let file_ext = match pkg.installer_type.as_str() {
        "exe" => "exe",
        "msi" => "msi",
        "zip" => "zip",
        "7z" => "7z",
        other => other,
    };

    let downloaded_pkg_path =
        packages_dir.join(format!("{}-{}.{}", pkg.name, pkg.version, file_ext));
    if !downloaded_pkg_path.exists() {
        if let Err(e) = download::download_file(&pkg.url, &downloaded_pkg_path) {
            eprintln!("Download failed: {}", e);
            return;
        }
        println!("{} download to {:?}", pkg.name, downloaded_pkg_path);
    } else {
        println!("installer already download, skipping...");
    }

    fs::create_dir_all(&apps_dir).unwrap();
    let app_install_dir = apps_dir.join(&pkg.name);
    if let Err(e) = installer::install_package(&downloaded_pkg_path, &app_install_dir, &pkg) {
        eprintln!("install failed: {}", e);
        return;
    }
    println!("{} installed to {:?}", pkg.name, app_install_dir);

    if let Err(e) = installed::add_installed_package(&pkg) {
        eprintln!("failed to write installed.json: {}", e);
    } else {
        println!("{} info added to installed.json", pkg.name);
    }
}