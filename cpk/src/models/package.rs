use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub installer_type: String,
    pub silent_args: String,
    pub env_path: String,
    pub size: f32,
    pub checksum: String,
    pub tags: Vec<String>,
}