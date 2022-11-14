use std::collections::HashMap;

use super::settings::global_settings;
use serde::{de, Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize)]
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
