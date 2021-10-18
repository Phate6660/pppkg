use crate::{GlobalVars, Package};
use log::{error, info, trace};
use std::path::Path;

/// Ensures that the directory specificed is created,
/// and return a usuable Path of the directory.
pub fn ensure_directory_and_return_path(dir: &str) -> &Path {
    trace!("Creating path '{}' and returning the usable type!", dir);
    let path = Path::new(dir);
    if !path.exists() {
        if let Err(e) = std::fs::create_dir(path) {
            error!("{}", e);
        } else {
            info!("Path was created successfully!");
        }
    }
    info!("Returning the usuable path!");
    path
}

/// Parses and serializes the config of the specified package.
pub fn parse_package(user_input: Vec<String>, pppkg_vars: &GlobalVars) -> Package {
    let file = [pppkg_vars.packages.to_str().unwrap(), "/", user_input[2].as_str(), ".toml"].concat();
    info!("Parsing package: {}", user_input[2]);
    toml::from_str(&std::fs::read_to_string(file).unwrap()).unwrap()
}

/// Used for getting the correct element from an arches or urls vector,
/// based on the length of the arches vector.
pub fn get_element(package: &Package, pppkg_vars: &GlobalVars) -> usize {
    if package.arches.len() == 2 {
        pppkg_vars.arch
    } else {
        pppkg_vars.arch - 1
    }
}
