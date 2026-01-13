use crate::models::package::Package;
use crate::utils::fs as utils_fs;
use serde_json::Value;

pub fn find_package(name: &str) -> Option<Package> {
    let exe_dir = utils_fs::get_exe_dir()?;
    let data_path = exe_dir.join("data").join("packages.json");
    let data = std::fs::read_to_string(data_path).ok()?;
    let json: Value = serde_json::from_str(&data).ok()?;
    let pkgs: Vec<Package> = serde_json::from_value(json.get("packages")?.clone()).ok()?;
    pkgs.into_iter().find(|p| p.name == name)
}