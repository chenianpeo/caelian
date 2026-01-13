use crate::models::package::Package;
use crate::utils::fs as utils_fs;
use serde_json;
use std::fs;

pub fn add_installed_package(pkg: &Package) -> Result<(), String> {
    let exe_dir = utils_fs::get_exe_dir().ok_or("cannot get exe dir")?;
    let file_path = exe_dir.join("data").join("installed.json");
    let mut installed = if file_path.exists() {
        let data = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    installed.push(pkg.clone());
    let data = serde_json::to_string_pretty(&installed).map_err(|e| e.to_string())?;
    fs::write(file_path, data).map_err(|e| e.to_string())
}