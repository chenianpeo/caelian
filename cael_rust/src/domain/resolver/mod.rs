mod conflict;
mod dependency_resolver;
mod graph;
mod result;

pub use crate::domain::resolver::result::{
    DecisionReason, DecisionRecord, ResolutionPlan, ResolvedPackage, ResolverResult,
};
