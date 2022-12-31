use dirs;
use serde::{de, ser, Deserialize, Serialize};
use serde_json::{from_str, from_value, json, to_string_pretty, Value};
use std::collections::HashMap;

use super::file;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GlobalSettings {
    #[serde(default = "get_config_dir")]
    pub working_dir: String,

    #[serde(default)]
    module_settings: HashMap<String, Value>,
}

impl GlobalSettings {
    pub fn get_module<T>(&self, module_name: &str) -> T
    where
        T: de::DeserializeOwned,
    {
        let module_settings = match self.module_settings.get(module_name) {
            Some(module_settings) => module_settings,
            None => panic!(
                "No settings found for module {module_name}",
                module_name = module_name
            ),
        };

        let module_settings: T = match from_value(module_settings.to_owned()) {
            Ok(settings) => settings,
            Err(error) => panic!("Unable to parse module settings due to error {:?}", error),
        };

        module_settings
    }

    pub fn insert_module<T: ser::Serialize>(&mut self, module_name: &str, settings: &T) -> &Self {
        let converted_settings = json!(settings);
        self.module_settings
            .insert(module_name.to_string(), converted_settings);
        self
    }

    pub fn write(&self) {
        let settings_str =
            to_string_pretty(self).expect("Unable to parse global settings for module {}");
        file::overwrite_file(&get_global_loc(), &settings_str)
    }

    pub fn module_exists(&self, module_name: &String) -> bool {
        self.module_settings.contains_key(module_name)
    }

    pub fn init_module<T: ser::Serialize>(&mut self, module_name: &str, default_settings: T) {
        if !self.module_exists(&module_name.to_string()) {
            self.insert_module(module_name, &default_settings);
        }
    }

    pub fn get() -> GlobalSettings {
        let global_settings_file_loc = get_global_loc();
        let global_settings_str = file::get_file_str(&global_settings_file_loc);

        match from_str(&global_settings_str) {
            Ok(global_settings) => global_settings,
            Err(error) => panic!("Unable to serialize settings due to error: {:?}", error),
        }
    }

    pub fn init() -> GlobalSettings {
        let file_loc = get_global_loc();
        if !file::file_exists(&file_loc) {
            let default_settings = GlobalSettings {
                ..Default::default()
            };

            file::create_parent_dirs(&file_loc);
            default_settings.write();

            default_settings
        } else {
            GlobalSettings::get()
        }
    }
}

fn get_global_loc() -> String {
    let global_settings_dir = get_config_dir();
    format!("{dir}/curld/settings.json", dir = global_settings_dir)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Default, Debug)]
    struct TestModule {
        #[serde(default)]
        pub name: String,
    }

    #[test]
    fn module_should_overwrite_module() {
        let test = TestModule {
            name: "test_name".to_string(),
        };

        let mut global_settings = GlobalSettings {
            working_dir: "".to_string(),
            module_settings: HashMap::from([("a".to_string(), json!(test))]),
        };

        let mut saved_test: TestModule = global_settings.get_module("a");

        saved_test.name = "mutated".to_string();

        global_settings.insert_module("a", &saved_test);
        println!("{:?}", global_settings);
    }
}
