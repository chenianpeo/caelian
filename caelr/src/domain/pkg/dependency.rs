use super::PackageID;
use crate::domain::rev::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub id: PackageID,
    pub version: Version,
}
