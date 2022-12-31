use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default};

use crate::{settings::global_settings::GlobalSettings, templates::TemplateBuilder};

pub static ENDPOINT_MODULE: &str = "endpoints";

#[derive(Deserialize, Serialize, Default)]
pub struct EndpointSettings {
    #[serde(default)]
    saved: HashMap<String, TemplateBuilder>,

    #[serde(default)]
    history_len: usize,

    #[serde(default)]
    history: Vec<String>,
}

impl EndpointSettings {
    pub fn add_saved(&mut self, id: String, history: TemplateBuilder) {
        self.saved.insert(id, history);
    }

    pub fn get_saved(&self, id: &String) -> Option<&TemplateBuilder> {
        self.saved.get(id)
    }

    pub fn get_saved_keys(&self) -> Vec<String> {
        self.saved.keys().map(|k| k.to_string()).collect()
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

    pub fn get() -> (EndpointSettings, GlobalSettings) {
        let global_settings = GlobalSettings::get();
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
}
