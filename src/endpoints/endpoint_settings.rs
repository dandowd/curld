use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default};

use crate::settings::global_settings::GlobalSettings;

pub static ENDPOINT_MODULE: &str = "endpoints";

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointSettings {
    #[serde(default)]
    saved: HashMap<String, SavedEndpoint>,

    #[serde(default)]
    history_len: usize,

    #[serde(default)]
    history: Vec<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct SavedEndpoint {
    #[serde(default)]
    pub endpoint: String,

    #[serde(default)]
    pub method: String,

    #[serde(default)]
    pub headers: Vec<String>,

    #[serde(default)]
    pub base_url: String,

    #[serde(default)]
    pub data: String,
}

impl EndpointSettings {
    pub fn add_saved(&mut self, id: String, history: SavedEndpoint) {
        self.saved.insert(id, history);
    }

    pub fn get_saved(&self, id: &String) -> Option<&SavedEndpoint> {
        self.saved.get(id)
    }

    pub fn get_saved_keys(&self) -> Vec<String> {
        self.saved
            .keys()
            .map(|k| k.to_string())
            .collect()
    }

    pub fn insert_history(&mut self, cmd: &str) {
        self.history.push(cmd.to_owned());
        self.history.truncate(self.history_len);
    }

    pub fn get_history_entries(&self) -> Vec<String> {
        self.history
            .iter()
            .enumerate()
            .map(|(index, value)| format!("{} | {}", index, value))
            .collect()
    }

    pub fn get_history_entry(&self, index: usize) -> Option<&String> {
        self.history.get(index)
    }
}

pub fn get_endpoint_settings() -> (EndpointSettings, GlobalSettings) {
    let global_settings = crate::global_settings::get_global_settings();
    let settings: EndpointSettings =
        global_settings.get_module(super::endpoint_settings::ENDPOINT_MODULE);

    (settings, global_settings)
}

pub fn default() -> EndpointSettings {
    EndpointSettings {
        history_len: 10,
        ..Default::default()
    }
}
