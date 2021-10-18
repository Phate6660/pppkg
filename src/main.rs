extern crate env_logger;

mod operations;
mod shared_functions;

use operations::{install, list, meta};
use serde_derive::Deserialize;
use shared_functions::{ensure_directory_and_return_path, parse_package};
use std::path::Path;

#[derive(Debug)]
/// Contains all of the necessary global variables for the package manager.
pub struct GlobalVars<'a> {
    // Arches:
    // - 0 is x86
    // - 1 is x86_64
    pub arch: usize,
    pub base: &'a Path,
    pub bin: &'a Path,
    pub downloads: &'a Path,
    pub opt: &'a Path,
    pub packages: &'a Path,
}

#[derive(Debug, Deserialize)]
/// This struct is used for serializing the config files for packages.
/// arches and urls are vectors as they both deal with multiple architectures.
pub struct Package {
    name: String,
    description: String,
    version: String,
    urls: Vec<String>,
    arches: Vec<String>,
}

fn main() {
    env_logger::init();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    let home = if home == "/home" {
        let user = std::env::var("USER").unwrap();
        [home, user, "/.pppkg".to_string()].concat()
    } else {
        [home, "/.pppkg".to_string()].concat()
    };
    let bin_dir = [&home, "/bin"].concat();
    let downloads_dir = [&home, "/downloads"].concat();
    let opt_dir = [&home, "/opt"].concat();
    let packages_dir = [&home, "/packages"].concat();
    let pppkg_vars = GlobalVars {
        #[cfg(target_arch = "x86")]
        arch: 0,
        #[cfg(target_arch = "x86_64")]
        arch: 1,
        base: ensure_directory_and_return_path(&home),
        bin: ensure_directory_and_return_path(&bin_dir),
        downloads: ensure_directory_and_return_path(&downloads_dir),
        opt: ensure_directory_and_return_path(&opt_dir),
        packages: ensure_directory_and_return_path(&packages_dir),
    };
    // Collect args passed to the program.
    let user_input: Vec<String> = std::env::args().collect();
    match user_input.get(1).unwrap_or(&"NOTHING".to_string()).as_str() {
        "i" | "install" => {
            let package = parse_package(user_input, &pppkg_vars);
            install(package, &pppkg_vars);
        }
        "l" | "list" => list(user_input, pppkg_vars),
        "m" | "meta" => {
            let package = parse_package(user_input, &pppkg_vars);
            meta(package, &pppkg_vars);
        },
        "NOTHING" => println!("Currently install, list, and meta are supported."),
        _ => println!("Sorry, {} is not a valid operation!", user_input[1]),
    }
}
