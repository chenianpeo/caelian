mod package;
mod identity;
mod metadata;
mod dependency;
mod constraint;
mod install;

pub use package::Package;
pub use identity::PackageID;
pub use metadata::{Description, License, Homepage, Author, Tag};
pub use dependency::Dependency;
pub use constraint::{PlatformConstraint, CapabilitySet, FeatureFlag};
pub use install::{InstallIntent, InstallScope};