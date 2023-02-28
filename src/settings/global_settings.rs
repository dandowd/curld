use dirs;
use serde::{de, Deserialize, Serialize};
use serde_json::{from_str, from_value, json, to_string_pretty, Value};
use std::collections::HashMap;

use super::traits::{Storage, StoredSettings};

pub struct GlobalSettings {
    storage: Box<dyn Storage>,
    settings: SerializeSettings,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SerializeSettings {
    #[serde(default = "get_config_dir")]
    pub working_dir: String,

    #[serde(default)]
    module_settings: HashMap<String, Value>,
}

impl<T: de::DeserializeOwned + Serialize> StoredSettings<T> for GlobalSettings {
    fn get_module(&self, module_name: &str) -> T {
        let module_settings = match self.settings.module_settings.get(module_name) {
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

    fn insert_module(&mut self, module_name: &str, settings: &T) {
        let converted_settings = json!(settings);
        self.settings
            .module_settings
            .insert(module_name.to_string(), converted_settings);
    }
}

impl GlobalSettings {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        let serialized_settings = match storage.get() {
            Some(global_settings) => {
                from_str(&global_settings).expect("Unable to serialize settings due to error")
            }
            None => SerializeSettings::default(),
        };

        Self {
            storage,
            settings: serialized_settings,
        }
    }

    pub fn write(&self) {
        let settings_str = to_string_pretty(&self.settings)
            .expect("Unable to parse global settings for module {}");

        self.storage.write(&settings_str);
    }
}

fn get_config_dir() -> String {
    let path_buf = match dirs::config_dir() {
        Some(dir) => dir,
        None => panic!("Unable to get config dir for OS"),
    };

    match path_buf.to_str() {
        Some(dir_str) => dir_str.to_owned(),
        None => panic!("Unable to convert config dir to string"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::traits::MockStorage;

    #[derive(Serialize, Deserialize, Default, Debug)]
    struct TestModule {
        #[serde(default)]
        pub name: String,
    }

    const SETTINGS: &str = r##"{
            "working_dir": "test/dir",
            "module_settings": {
                "test_module": {
                    "name": "test module"
                }
            }
        }"##;

    #[test]
    fn should_try_to_load_settings_from_storage_without_panicing() {
        let mut mock_storage = Box::new(MockStorage::new());
        mock_storage
            .expect_get()
            .once()
            .returning(move || Some(String::from(SETTINGS)));
        GlobalSettings::new(mock_storage);
    }

    #[test]
    fn should_insert_module() {
        let mut mock_storage = Box::new(MockStorage::new());
        mock_storage
            .expect_get()
            .once()
            .returning(move || Some(String::from(SETTINGS)));
        let mut global_settings = GlobalSettings::new(mock_storage);

        let module = to_string_pretty(&TestModule {
            name: "inserted_module".to_string(),
        })
        .expect("error while trying to construct test module");

        global_settings.insert_module("inserted_module", &module);
    }

    #[test]
    fn should_retrieve_test_module() {
        let mut mock_storage = Box::new(MockStorage::new());
        mock_storage
            .expect_get()
            .once()
            .returning(move || Some(String::from(SETTINGS)));
        let global_settings = GlobalSettings::new(mock_storage);

        let module: TestModule = global_settings.get_module("test_module");

        assert_eq!(module.name, "test module");
    }
}
