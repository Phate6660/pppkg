use compress_tools::*;
use crate::{GlobalVars, Package};
use crate::shared_functions::get_element;
use log::info;

pub fn list(user_input: Vec<String>, pppkg_vars: GlobalVars) {
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

pub fn install(package: Package, pppkg_vars: &GlobalVars) {
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

pub fn meta(package: Package, pppkg_vars: &GlobalVars) {
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
