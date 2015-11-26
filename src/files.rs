use std::io::Read;
use std::io::Write;
use std::fs::File;

pub fn read_text_file(file_path: &str) -> Result<String, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }

    Ok(contents)
}

pub fn create_text_file(file_path: &str, contents: &str) -> Result<(), String> {
    let mut file = match File::create(file_path) {
                Ok(file) => file,
                Err(err) => return Err(err.to_string()),
            };

    match file.write_all(contents.as_bytes()) {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    }

    Ok(())
}
