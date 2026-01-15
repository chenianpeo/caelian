use std::{
    fs::File,
    path::PathBuf,
    io::Read,
};
use serde::Deserialize;
use crate::packages_list::packages_list_struct::PackagesInfo;

pub fn packages_list_obtain() {
    // create a variable PathBuf argument
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // add child path from current path start
    path.push("src/packages_data/packages.yaml");

    // try open relevant file, output error info if opening occur error
    let mut f = File::open(&path).expect("Failed to open packages.yaml");
    // create a empty variable string argument
    let mut content = String::new();
    // use content read yaml file content
    f.read_to_string(&mut content).expect("Failed to read file");
    
    // create a yaml deserializer to analysis yaml file content
    // use --- split if this yaml contain multiple yaml document
    let deserializer = serde_yaml::Deserializer::from_str(&content);
    // ergodic yaml file content and print relevant content
    for document in deserializer {
        // use PackagesInfo format deserialize document content
        match PackagesInfo::deserialize(document) {
            Ok(pkg) => println!("{:#?}", pkg),
            Err(_) => continue,
        }
    }
}