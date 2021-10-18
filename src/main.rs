extern crate env_logger;
use compress_tools::*;
use log::{error, info, trace};
use serde_derive::Deserialize;
use std::path::Path;

fn ensure_directory_and_return_path(dir: &str) -> &Path {
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

#[derive(Debug)]
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
struct Package {
    name: String,
    description: String,
    version: String,
    urls: Vec<String>,
    arches: Vec<String>,
}

fn list(user_input: Vec<String>, pppkg_vars: GlobalVars) {
    match user_input[2].as_str() {
        "-a" | "--available" => {
            let files = std::fs::read_dir(pppkg_vars.packages.to_str().unwrap()).unwrap();
            for file in files {
                println!("{}", file.unwrap().path().display());
            }
        }
        _ => println!("Sorry, {} is not a valid operation.", user_input[1]),
    }
}

fn parse_package(user_input: Vec<String>, pppkg_vars: &GlobalVars) -> Package {
    let file = [pppkg_vars.packages.to_str().unwrap(), "/", user_input[2].as_str(), ".toml"].concat();
    info!("Parsing package: {}", user_input[2]);
    toml::from_str(&std::fs::read_to_string(file).unwrap()).unwrap()
}

fn get_element(package: &Package, pppkg_vars: &GlobalVars) -> usize {
    if package.arches.len() == 2 {
        pppkg_vars.arch
    } else {
        pppkg_vars.arch - 1
    }
}

fn install(package: Package, pppkg_vars: &GlobalVars) {
    let element = get_element(&package, pppkg_vars);
    let url = &package.urls[element];
    let response = reqwest::blocking::get(url).expect("failed to download tarball");

    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let full_path = [pppkg_vars.downloads.to_str().unwrap(), "/", fname].concat();
    info!("Downloading to: {}", full_path);

    let mut dest = std::fs::File::create(&full_path).expect("failed to create file"); // Create the file
    std::io::copy(
        &mut std::io::Cursor::new(response.bytes().unwrap()),
        &mut dest,
    ).unwrap(); // Stick the contents in the file

    info!("Downloading finished! Starting to extract.");
    let mut source = std::fs::File::open(&full_path).expect("could not open archive");
    let extraction_path = [pppkg_vars.opt.to_str().unwrap(), "/", &package.name].concat();
    let dest = std::path::Path::new(&extraction_path);
    uncompress_archive(&mut source, dest, Ownership::Ignore).expect("could not unpack archive");
}

fn meta(package: Package, pppkg_vars: &GlobalVars) {
    let element = get_element(&package, pppkg_vars);
    // Grab the right element from the vector based on the arch
    println!("\
        {} ({}) [{}]\n\
        ----\n\
        {}\n\
        {}\
    ", package.name, package.version, package.arches[element], 
    package.description, 
    package.urls[element]);
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
