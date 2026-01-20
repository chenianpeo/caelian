mod constraint;
mod dependency;
mod identity;
mod install;
mod metadata;
mod package;

pub use constraint::{CapabilitySet, FeatureFlag, PackageSource, PlatformConstraint};
pub use dependency::Dependency;
pub use identity::PackageID;
pub use install::{InstallIntent, InstallScope};
pub use metadata::{Author, Description, Homepage, License, Tag};
pub use package::Package;
