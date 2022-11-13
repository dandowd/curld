use std::fs::File;
use std::io::prelude::*;

/// Overwrites the entire content of a file with the provided string
///
/// # Panics
/// If there is an error while getting the file or writing to the file
/// Panics if .
pub fn overwrite_file(file_loc: &String, content: &String) {
    let mut file = get_file(file_loc);
    match file.write_all(content.as_bytes()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to overwrite file due to {:?}", error),
    };
}

/// Reads entire contents of a file into a string
///
/// # Panics
/// If there is an error while getting the file or reading the file
/// Panics if .
pub fn get_file_str(file_loc: &String) -> String {
    let mut file = get_file(file_loc);
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content_str) => content_str,
        Err(error) => panic!("Unable to read content of file due to {:?}", error),
    };
    content
}

/// Retrieves a readable and writable file. It will create the file if it does not exist
///
/// # Panics
/// If there is a problem while opening the file
/// Panics if .
fn get_file(file_loc: &String) -> File {
    let file_result = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(file_loc);
    match file_result {
        Ok(file) => file,
        Err(error) => panic!("Error occurred while tyring to overwrite file: {:?}", error),
    }
}
