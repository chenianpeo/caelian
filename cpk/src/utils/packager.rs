use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("Building release binary...");
    let status = Command::new("cargo")
    .args(&["build", "--release"])
    .status()
    .expect("failed to build project");

    if !status.success() {
        eprintln!("cargo build failed");
        std::process::exit(1);
    }

    let exe_name = if cfg!(windows) {
        "cpk.exe"
    } else {
        "cpk"
    };

    let binary_src = PathBuf::from("target/release").join(exe_name);
    let dist_dir = Path::new("dist");
    let data_src = Path::new("data/packages.json");
    let data_dst_dir = dist_dir.join("data");

    fs::create_dir_all(&data_dst_dir).expect("failed to create dist/data directory");

    let binary_dst = dist_dir.join(exe_name);
    fs::copy(&binary_src, &binary_dst).expect("failed to copy binary");

    fs::copy(&data_src, &data_dst_dir.join("packages.json"))
    .expect("failed to copy binary");

    println!("package to dist/ right");
}