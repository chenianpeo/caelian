use crate::domain::package_impl::PackageID;
use crate::domain::package_impl::PackageSource;
use crate::domain::package_impl::PlatformConstraint;
use crate::domain::version_impl::{Version, VersionRequirement};

pub enum ResolverResult {
    Resolved(ResolutionPlan),
    Unresolved(ResolutionFailure),
}

// resolver success
pub struct ResolutionPlan {
    pub install_order: Vec<PackageID>,  // install order
    pub packages: Vec<ResolvedPackage>, // packages list
    pub decisions: Vec<DecisionRecord>, // decision record
}

pub struct ResolvedPackage {
    pub id: PackageID,
    pub version: Version,
    pub source: PackageSource,
}

pub struct DecisionRecord {
    pub package_id: PackageID,
    pub reason: DecisionReason,
}

// decision reason enum
pub enum DecisionReason {
    RootRequest,
    DependencyOf(PackageID),
    VersionSelected {
        requirement: VersionRequirement,
        selected: Version,
    },
}

// resolver failure
pub struct ResolutionFailure {
    pub conflicts: Vec<DependencyConflict>, // conflict list
}

pub enum DependencyConflict {
    VersionConflict {
        package: PackageID,
        requested_by: PackageID,
        requirement: VersionRequirement,
        available: Vec<Version>,
    },

    CycleDetected {
        cycle: Vec<PackageID>,
    },

    PlatformMismatch {
        package: PackageID,
        platform: PlatformConstraint,
    },

    MissingDependency {
        package: PackageID,
        dependency: PackageID,
    },
}
