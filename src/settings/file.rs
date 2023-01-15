use std::fs::File;
use std::io::prelude::*;

use super::traits::Storage;

pub struct FileStorage {
    settings_file_path: String,
}

impl FileStorage {
    pub fn new(file_path: Option<&str>) -> Box<FileStorage> {
        if let Some(directory) = file_path {
            Box::new(Self {
                settings_file_path: directory.to_string(),
            })
        } else {
            Box::new(Self {
                settings_file_path: get_settings_file_loc(),
            })
        }
    }
}

impl Storage for FileStorage {
    fn write(&self, content: &str) {
        overwrite_file(&self.settings_file_path, content)
    }

    fn get(&self) -> String {
        if file_exists(&self.settings_file_path) {
            get_file_str(&self.settings_file_path)
        } else {
            create_parent_dirs(&self.settings_file_path);

            get_file_str(&self.settings_file_path)
        }
    }
}

fn get_settings_file_loc() -> String {
    let global_settings_dir = get_config_dir();
    format!("{dir}/curld/settings.json", dir = global_settings_dir)
}

fn get_config_dir() -> String {
    let path = dirs::config_dir().expect("Unable to OS config dir");

    path.to_str()
        .expect("Unable to convert config dir to string")
        .to_owned()
}
pub fn overwrite_file(file_loc: &String, content: &str) {
    let mut file = get_file(file_loc);
    match file.write_all(content.as_bytes()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to overwrite file due to {:?}", error),
    };
}

pub fn get_file_str(file_loc: &String) -> String {
    let mut file = get_file(file_loc);
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content_str) => content_str,
        Err(error) => panic!("Unable to read content of file due to {:?}", error),
    };
    content
}

pub fn file_exists(file_loc: &String) -> bool {
    std::path::Path::new(file_loc).exists()
}

pub fn create_parent_dirs(file_loc: &String) {
    let path = std::path::Path::new(file_loc);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Unable to create directory for settings");
}

fn get_file(file_loc: &String) -> File {
    let file_result = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(file_loc);
    match file_result {
        Ok(file) => file,
        Err(error) => panic!("Error occurred while trying to open file: {:?}", error),
    }
}
