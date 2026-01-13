use crate::models::package::Package;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDatabase {
    pub packages: Vec<Package>,
}