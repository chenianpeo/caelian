use crate::domain::{
    pkg::{Dependency, PackageID, Platform},
    rev::Version,
};

#[derive(Debug)]
pub struct Package {
    pub id: PackageID,
    pub version: Version,

    pub dependencies: Vec<Dependency>,

    pub platform: Platform,
}

impl Package {
    pub fn new(
        id: PackageID,
        version: Version,
        dependencies: Vec<Dependency>,
        platform: Platform,
    ) -> Self {
        Self {
            id,
            version,
            dependencies,
            platform,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rev::Version;

    // test package format
    #[test]
    fn test_create_package_windows() {
        let package_id = PackageID {
            name: "windows_package".into(),
        };
        let version: Version = Version::new(1, 2, 5);
        let dependencies = vec![];
        let platform: Platform = Platform::Windows(crate::domain::pkg::WindowsArch::X86_64);
        let package: Package = Package::new(package_id, version, dependencies, platform);

        assert_eq!(package.id.name, "windows_package");
        println!("{:#?}", package);
    }

    #[test]
    fn test_package_dependencies() {
        let package_id = PackageID {
            name: "first_package".into(),
        };
        let version = Version::new(2, 5, 10);

        let dep_one = Dependency {
            id: PackageID {
                name: "dep1".into(),
            },
            version: Version::new(1, 2, 3),
        };
        let dep_two = Dependency {
            id: PackageID {
                name: "dep2".into(),
            },
            version: Version::new(3, 4, 18),
        };

        let platform = Platform::Linux(crate::domain::pkg::LinuxPackageFormat::Deb);
        let package = Package::new(package_id, version, vec![dep_one, dep_two], platform);
        println!("{:#?}", package);
    }
}
