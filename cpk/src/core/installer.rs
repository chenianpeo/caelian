use crate::models::package::Package;
use std::path::Path;

pub fn install_package(installer: &Path, dest: &Path, pkg: &Package) -> Result<(), String> {
    // Create destination directory if it doesn't exist
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    
    if pkg.installer_type == "exe" {
        let status = std::process::Command::new(installer)
            .arg(&pkg.silent_args)
            .current_dir(dest)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!("Installer returned error: {:?}", status));
        }
    } else if pkg.installer_type == "zip" {
        // TODO decompress to dest
    }
    Ok(())
}
