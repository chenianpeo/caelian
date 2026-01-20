use super::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionRequirement {
    Any,
    Exact(Version),
    GreaterThan(Version),
    GreaterOrEqual(Version),
    LessThan(Version),
    LessOrEqual(Version),
}

impl VersionRequirement {
    pub fn satisfies(&self, version: &Version) -> bool {
        match self {
            VersionRequirement::Any => true,

            VersionRequirement::Exact(v) => version == v,

            VersionRequirement::GreaterThan(v) => version > v,

            VersionRequirement::GreaterOrEqual(v) => version >= v,

            VersionRequirement::LessThan(v) => version < v,

            VersionRequirement::LessOrEqual(v) => version <= v,
        }
    }
}
