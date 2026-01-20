use super::constraint::{CapabilitySet, FeatureFlag, PlatformConstraint};
use super::dependency::Dependency;
use super::identity::PackageID;
use super::install::{InstallIntent, InstallScope};
use super::metadata::{Author, Description, Homepage, License, Tag};
use crate::domain::version::Version;

pub struct Package {
    pub id: PackageID,
    pub version: Version,

    pub description: Description,
    pub license: License,
    pub homepage: Homepage,
    pub author: Author,
    pub tags: Vec<Tag>,

    pub dependencies: Vec<Dependency>,

    pub capabilities: CapabilitySet,
    pub platform: PlatformConstraint,
    pub features: FeatureFlag,

    pub install_scope: InstallScope,
    pub install_intent: InstallIntent,
}
