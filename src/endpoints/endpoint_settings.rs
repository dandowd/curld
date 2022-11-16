use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointSettings {
    #[serde(default)]
    history: Vec<EndpointHistory>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointHistory {
    #[serde(default)]
    pub headers: Vec<String>,

    #[serde(default)]
    pub base_url: String,

    #[serde(default)]
    pub endpoint: String,

    #[serde(default)]
    pub data: String,
}

pub fn default() {
    let default_endpoint_settings = EndpointSettings {
        ..Default::default()
    };
}
