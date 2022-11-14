use dirs;
use serde::{de, ser, Deserialize, Serialize};
use serde_json::{from_str, from_value, json, to_string_pretty, Value};
use std::collections::HashMap;

use super::file;

#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(default = "get_config_dir")]
    pub working_dir: String,

    #[serde(default)]
    pub module_settings: HashMap<String, Value>,
}

impl GlobalSettings {
    /// Gets module's settings
    ///
    /// # Panics
    /// If module has not created default settings, or if the module settings cannot be parsed
    /// Panics if .
    pub fn get_module<T: de::DeserializeOwned>(module_name: &String) -> T {
        let global_settings = get();
        let module_settings = match global_settings.module_settings.get(module_name) {
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

    pub fn set_module<T: ser::Serialize>(module_name: &String, settings: T) {
        let mut global_settings = get();
        let converted_settings = json!(settings);
        global_settings
            .module_settings
            .insert(module_name.to_string(), converted_settings);

        let content_str = to_string_pretty(&global_settings).expect(&format!(
            "Unable to parse settings for module {}",
            module_name
        ));
        file::overwrite_file(&get_global_loc(), &content_str)
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
    format!("{dir}/curld-settings.json", dir = global_settings_dir)
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
