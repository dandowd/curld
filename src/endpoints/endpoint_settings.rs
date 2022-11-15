use crate::settings::global_settings::GlobalSettings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointSettings {
    #[serde(default)]
    pub headers: Vec<String>,

    #[serde(default)]
    pub base_url: String,

    #[serde(default)]
    pub endpoint: String,

    #[serde(default)]
    pub data: String,
}

pub fn init(global_settings: &mut GlobalSettings) {
    if !global_settings.module_exists(&String::from("endpoints")) {
        global_settings.insert_module::<Vec<EndpointSettings>>(&"endpoints".to_string(), Vec::new());
    }
}
