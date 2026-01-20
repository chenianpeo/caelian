use crate::domain::package::PackageID;
use crate::domain::package::PlatformConstraint;
use crate::domain::version::{VersionRequirement, Version};

pub enum ResolverResult {
    Resolved(ResolutionPlan),
    Unresolved(ResolutionFailure),
}

pub struct ResolutionPlan {
    pub install_order: Vec<PackageID>,
    pub packages: Vec<ResolvedPackage>,
    pub decisions: Vec<DecisionRecord>,
}

pub struct ResolvedPackage {
    pub id: PackageID,
    pub version: Version,
    pub source: PackageSource,
}

pub enum PackageSource {
    Registry,
    Git,
    Local,
}

pub struct DecisionRecord {
    pub package_id: PackageID,
    pub reason: DecisionReason,
}

pub enum DecisionReason {
    RootRequest,
    DependencyOf(PackageID),
    VersionSelected {
        requirement: VersionRequirement,
        selected: Version,
    }
}

pub struct ResolutionFailure {
    pub conflicts: Vec<DependencyConflict>,
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
    }
}