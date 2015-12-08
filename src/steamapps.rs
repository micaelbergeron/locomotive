extern crate glob;

use std::io;
use std::io::prelude::*;
use std::fs::{File};
use std::path::{PathBuf, Path};

use acf::{Manifest, ValueExpression, parse};

#[derive(Debug)]
pub struct SteamLibrary {
    pub manifests: Vec<Manifest>,
    path: PathBuf,
}

impl SteamLibrary {
    pub fn new(library_path: &Path) -> SteamLibrary {
        SteamLibrary { manifests: Vec::new(), path: PathBuf::from(library_path) }
    }

    pub fn find_libraries(&self) -> io::Result<Vec<PathBuf>> {
        let lib_file_path = self.path.join("libraryfolders.vdf");
        println!("Looking in {:?} for library folder.", lib_file_path);

        let library_vdf = Self::read_manifest(&lib_file_path).unwrap();
        println!("{:?}", library_vdf);    

        let paths = library_vdf.bundle
            .values()
            .filter_map(|value| match **value {
                ValueExpression::Path(ref p) => Some(p.clone()), 
                _ => None
            })
            .collect();
        Ok(paths)
    }

    pub fn load_manifests(&mut self) -> io::Result<usize> {
        let pattern: glob::Pattern = glob::Pattern::new("*.acf").unwrap();
        let iter = try!(self.path.read_dir())
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry.path()),
                _ => None
            })
            .filter(|p| pattern.matches_path(p));

        for manifest_file in iter {            
            println!("{:?}", manifest_file);
            self.manifests.push(Self::read_manifest(&manifest_file).unwrap());
        }
        Ok(self.manifests.len())
    }

    fn read_manifest(file_path: &Path) -> io::Result<Manifest> {
        let mut manifest_file = try!(File::open(file_path));
        let mut buf = String::new();

        try!(manifest_file.read_to_string(&mut buf));
        Ok(parse(&buf).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    const DEFAULT_LIB: &'static str = "/home/micael/.steam/steam/steamapps";

    #[test]
    fn has_default_steam_library() {
        let default_lib = SteamLibrary::new(Path::new(DEFAULT_LIB));
        assert_eq!(default_lib.find_libraries().ok().unwrap(), [Path::new("/mnt/steam/SteamLibrary")]);
    }

    #[test]
    fn has_game_manifests() {
        let mut default_lib = SteamLibrary::new(Path::new(DEFAULT_LIB));
        assert!(default_lib.load_manifests().is_ok());
        for manifest in default_lib.manifests {
            println!("Found game: {:?}", manifest.bundle.get("name").unwrap());
        }
        panic!();
    }
}
      
