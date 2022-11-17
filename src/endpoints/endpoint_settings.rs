use serde::{Deserialize, Serialize};
use std::{default::Default, collections::HashMap};

pub static ENDPOINT_MODULE: &str = "endpoints";

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointSettings {
    #[serde(default)]
    saved: HashMap<String, SavedEndpoint>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct SavedEndpoint {
    #[serde(default)]
    pub endpoint: String,

    #[serde(default)]
    pub method: String,

    #[serde(default)]
    pub headers: Option<Vec<String>>,

    #[serde(default)]
    pub base_url: Option<String>,

    #[serde(default)]
    pub data: Option<String>,
}

impl EndpointSettings {
    pub fn add_saved(&mut self, id: String, history: SavedEndpoint) {
        self.saved.insert(id, history); 
    }

    pub fn get_saved(&self, id: &String) -> Option<&SavedEndpoint> {
        self.saved.get(id)
    }
}

pub fn default() -> EndpointSettings {
    EndpointSettings {
        ..Default::default()
    }
}
