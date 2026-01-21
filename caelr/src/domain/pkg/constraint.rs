#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    Windows(WindowsArch),
    Linux(LinuxPackageFormat),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsArch {
    X86_64,
    Arm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinuxPackageFormat {
    Deb,
    Rpm,
}
