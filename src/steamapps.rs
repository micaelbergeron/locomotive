use std::fs::{File, ReadDir};
use std::path::{PathBuf, Path};

pub fn find_libraries() -> Vec<PathBuf> {
    let DEFAULT_LIB = "~/.steam/steam/steamapps/";
    let mut paths = Vec::new();

    paths.push(PathBuf::from(DEFAULT_LIB));
    return paths;
}
