use std::collections::HashMap;

use super::settings::global_settings;
use serde::{de, Deserialize, Serialize};
use serde_json::{Map, Value};

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

pub fn init() {
    let mut settings = global_settings::get();
    if !settings.module_exists(&String::from("endpoints")) {
        settings.set_module::<Vec<EndpointSettings>>(&"endpoints".to_string(), Vec::new());
    }
}
