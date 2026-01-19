#[derive(Debug, PartialEq)]
pub enum Installability {
    Installable,
    NotInstallable(&'static str),
}

use crate::domain::model::main_mod::Package;
/// exam package mod
impl Package {
    /// exam package validity
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
        && !self.version.is_empty()
    }

    /// exam deps rationality
    pub fn has_self_dependency(&self) -> bool {
        self.dependencies.iter().any(|dep| dep == &self.name)
    }

    pub fn check_installable(&self) -> Installability {
        if !self.is_valid() {
            return Installability::NotInstallable("invalid package metadata");
        }

        if self.has_self_dependency() {
            return Installability::NotInstallable("self dependency detected");
        }

        Installability::Installable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_package() {
        let pkg = Package {
            name: "get".into(),
            version: "2.2.2".into(),
            size: 1.2,
            dependencies: vec![],
        };

        assert_eq!(pkg.check_installable(), Installability::Installable);
    }
}