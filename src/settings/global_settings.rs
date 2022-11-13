use std::collections::HashMap;
use serde::{Serialize, Deserialize,de};
use serde_json::{Value, from_str, from_value};
use dirs;

use super::file;

#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(default = "get_config_dir")]
    pub working_dir: String, 

    #[serde(default)]
    pub module_settings: HashMap<String, Value>
}

pub fn get_module_settings<T: de::DeserializeOwned>(module_name: &String) -> T {
   let global_settings = get_global_settings();
   let module_settings = match global_settings.module_settings.get(module_name) {
        Some(module_settings) => module_settings,
        None => panic!("Module {module_name} has not initialized it's settings", module_name = module_name)
   };

   let module_settings: T = match from_value(module_settings.to_owned()) {
       Ok(settings) => settings,
       Err(error) => panic!("Unable to parse module settings due to error {:?}", error)
   };

   module_settings
} 

pub fn get_global_settings() -> GlobalSettings {
    let global_settings_dir = get_config_dir();
    let global_settings_file = format!("{dir}/curld-settings.json", dir = global_settings_dir);
    let global_settings_str = file::get_file_str(&global_settings_file);

    match from_str(&global_settings_str) {
        Ok(global_settings) => global_settings,
        Err(error) => panic!("Unable to serialize settings due to error: {:?}", error)
    }
}

fn get_config_dir() -> String {
    let path_buf = match dirs::config_dir() {
        Some(dir) => dir,
        None => panic!("Unable to OS config dir")
    };

    match path_buf.to_str() {
        Some(dir_str) => dir_str.to_owned(),
        None => panic!("Unable to convert config dir to string")
    }
}
