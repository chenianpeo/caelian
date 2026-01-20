pub struct CapabilitySet {}

pub enum PlatformConstraint {}

pub struct FeatureFlag {}
// FeatureFlag should is a set value object

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    Registry,
    Git,
    Local,
}
