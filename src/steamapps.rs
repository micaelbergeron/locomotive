use std::io;
use std::io::prelude::*;
use std::fs::{File, ReadDir};
use std::path::{PathBuf, Path};
use std::collections::HashMap;

use acf::{ValueExpression, parse};

pub fn find_libraries() -> io::Result<Vec<PathBuf>> {
    let DEFAULT_LIB = "/home/micael/.steam/steam/steamapps";

    let lib_file_path = Path::new(DEFAULT_LIB).join("libraryfolders.vdf");
    println!("Looking in {:?} for library folder.", lib_file_path);
    
    let mut library_file = try!(File::open(lib_file_path));
    let mut buf = String::new();

    try!(library_file.read_to_string(&mut buf));
    let library_vdf = parse(&buf).unwrap();

    println!("{:?}", library_vdf);    

    let paths = library_vdf.bundle
        .values()
        .filter_map(|value| match **value {
            ValueExpression::Path(ref p) => Some(p.clone()), 
            _ => None
        })
        .collect();
    return Ok(paths);
}
