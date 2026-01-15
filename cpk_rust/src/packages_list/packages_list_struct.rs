use serde::{Serialize, Deserialize};

/// installable packages information
#[derive(Debug, Serialize, Deserialize)]
pub struct PackagesInfo {
    pub name: String,
    pub version: String,
    pub url: String,
    pub size: Size,
    pub checksum: Option<Checksum>,
    pub platform: String,
    pub description: String,
}

/// packages size
#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub human: f64,
    pub bytes: usize,
}

/// packages verify
#[derive(Debug, Serialize, Deserialize)]
pub struct Checksum {
    pub sha256: Option<String>
}