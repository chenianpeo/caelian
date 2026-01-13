use std::env;
use std::path::{PathBuf, Path};
use std::io;

pub fn get_exe_dir() -> Option<PathBuf> {
    env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    std::fs::create_dir_all(path)
}