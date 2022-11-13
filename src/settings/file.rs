use std::fs::{File};
use std::io::prelude::*;

pub fn overwrite_file(file_loc: &String, content: &String) {
    let mut file = get_file(file_loc);
    match file.write_all(content.as_bytes()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to overwrite file due to {:?}", error)
    }; 
}

pub fn get_file_str(file_loc: &String) -> String {
    let mut file = get_file(file_loc);
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content_str) => content_str,
        Err(error) => panic!("Unable to read content of file due to {:?}", error)
    };
    content
}

fn get_file(file_loc: &String) -> File {
    let file_result = File::options().create(true).write(true).read(true).open(file_loc);
    match file_result {
        Ok(file) => file,
        Err(error) => panic!("Error occurred while tyring to overwrite file: {:?}", error)
    }
}
