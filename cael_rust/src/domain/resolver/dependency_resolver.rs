use std::collections::{HashMap, HashSet};

use crate::domain::package_impl::{Package, PackageID};
use crate::domain::resolver::result::{
    DecisionReason, DecisionRecord, ResolutionPlan, ResolvedPackage, ResolverResult,
};

#[allow(dead_code)]
pub fn resolve(packages: Vec<Package>) -> ResolverResult {
    let mut install_order: Vec<PackageID> = Vec::new();
    let mut resolved_packages: Vec<ResolvedPackage> = Vec::new();
    let mut decisions: Vec<DecisionRecord> = Vec::new();
    let mut installed: HashSet<PackageID> = HashSet::new();

    let mut dependency_graph: HashMap<PackageID, Vec<PackageID>> = HashMap::new();

    for pkg in packages.into_iter() {
        let id = pkg.id.clone();
        let version = pkg.version.clone();

        if installed.contains(&id) {
            continue;
        }

        // analysis dependency relation
        let dependencies = get_dependencies(&pkg);

        dependency_graph.insert(id.clone(), dependencies);

        install_order.push(id.clone());

        resolved_packages.push(ResolvedPackage {
            id: id.clone(),
            version,
            source: pkg.source.clone(),
        });

        decisions.push(DecisionRecord {
            package_id: id.clone(),
            reason: DecisionReason::RootRequest,
        });
        installed.insert(id);
    }

    let plan = ResolutionPlan {
        install_order,
        packages: resolved_packages,
        decisions,
    };

    ResolverResult::Resolved(plan)
}

#[allow(dead_code)]
fn get_dependencies(pkg: &Package) -> Vec<PackageID> {
    pkg.dependencies.iter().map(|dep| dep.id.clone()).collect()
}
