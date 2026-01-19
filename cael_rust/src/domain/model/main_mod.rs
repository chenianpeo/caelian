/// install packages meta data
pub struct Package {
    pub name: String,
    pub version: String,
    pub size: f32,
    pub dependencies: Vec<String>,
}