extern crate env_logger;
use log::{error, info, trace};
use std::path::Path;

fn ensure_directory_and_return_path (dir: &str) -> &Path {
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
    pub base: &'a Path,
    pub bin: &'a Path,
    pub downloads: &'a Path,
    pub opt: &'a Path,
    pub packages: &'a Path
}

fn main() {
    env_logger::init();
    let home = std::env::var("HOME").unwrap_or("/home".to_string());
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
        base: &ensure_directory_and_return_path(&home),
        bin: &ensure_directory_and_return_path(&bin_dir),
        downloads: &ensure_directory_and_return_path(&downloads_dir),
        opt: &ensure_directory_and_return_path(&opt_dir),
        packages: &ensure_directory_and_return_path(&packages_dir)
    };
    println!("{:#?}", pppkg_vars);
}
