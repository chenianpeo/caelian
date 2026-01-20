use crate::domain::package_impl::PackageID;
use crate::domain::version_impl::VersionRequirement;

pub struct Dependency {
    pub id: PackageID,
    pub requirement: VersionRequirement,
}
