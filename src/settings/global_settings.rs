use dirs;
use serde::{de, ser, Deserialize, Serialize};
use serde_json::{from_str, from_value, json, to_string_pretty, Value};
use std::collections::HashMap;

use super::file;

#[derive(Serialize, Deserialize, Default)]
pub struct GlobalSettings {
    #[serde(default = "get_config_dir")]
    pub working_dir: String,

    #[serde(default)]
    module_settings: HashMap<String, Value>,
}

impl GlobalSettings {
    /// Gets module's settings
    ///
    /// # Panics
    /// If module has not created default settings, or if the module settings cannot be parsed
    /// Panics if .
    pub fn get_module<T: de::DeserializeOwned>(&self, module_name: &String) -> T {
        let module_settings = match self.module_settings.get(module_name) {
            Some(module_settings) => module_settings,
            None => panic!(
                "Module {module_name} has not initialized it's settings",
                module_name = module_name
            ),
        };

        let module_settings: T = match from_value(module_settings.to_owned()) {
            Ok(settings) => settings,
            Err(error) => panic!("Unable to parse module settings due to error {:?}", error),
        };

        module_settings
    }

    pub fn insert_module<T: ser::Serialize>(&mut self, module_name: &String, settings: T) {
        let converted_settings = json!(settings);
        self.module_settings
            .insert(module_name.to_string(), converted_settings);
    }

    pub fn write(&self) {
        let settings_str =
            to_string_pretty(self).expect(&"Unable to parse global settings for module {}");
        file::overwrite_file(&get_global_loc(), &settings_str)
    }

    pub fn module_exists(&self, module_name: &String) -> bool {
        self.module_settings.contains_key(module_name)
    }
}

pub fn init() -> GlobalSettings {
    if !file::file_exists(&get_global_loc()) {
        let default_settings = GlobalSettings {
            ..Default::default()
        };
        default_settings.write();

        default_settings
    } else {
        get()
    }
}

/// Gets global settings
///
/// # Panics
/// If global settings cannot be parsed
/// Panics if .
pub fn get() -> GlobalSettings {
    let global_settings_file_loc = get_global_loc();
    let global_settings_str = file::get_file_str(&global_settings_file_loc);

    match from_str(&global_settings_str) {
        Ok(global_settings) => global_settings,
        Err(error) => panic!("Unable to serialize settings due to error: {:?}", error),
    }
}

fn get_global_loc() -> String {
    let global_settings_dir = get_config_dir();
    format!("{dir}/curlme/settings.json", dir = global_settings_dir)
}

/// Gets config dir and converts path_buf to string
///
/// # Panics
/// If the OS config directory cannot be found or path_buf cannot be converted to string
/// Panics if .
fn get_config_dir() -> String {
    let path_buf = match dirs::config_dir() {
        Some(dir) => dir,
        None => panic!("Unable to OS config dir"),
    };

    match path_buf.to_str() {
        Some(dir_str) => dir_str.to_owned(),
        None => panic!("Unable to convert config dir to string"),
    }
}
