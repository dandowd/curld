use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default};

use crate::settings::traits::StoredSettings;

use super::TemplateBuilder;

pub static RUN_MODULE: &str = "run";

pub struct RunSettings {
    parent: Box<dyn StoredSettings<SerializedSettings>>,

    settings: SerializedSettings,
}

#[derive(Deserialize, Serialize, Default)]
struct SerializedSettings {
    #[serde(default)]
    saved: HashMap<String, TemplateBuilder>,

    #[serde(default)]
    history_len: usize,

    #[serde(default)]
    history: Vec<TemplateBuilder>,
}

impl RunSettings {
    pub fn add_saved(&mut self, id: String, history: TemplateBuilder) {
        self.settings.saved.insert(id, history);
    }

    pub fn get_saved(&self, id: &String) -> Option<&TemplateBuilder> {
        self.settings.saved.get(id)
    }

    pub fn get_saved_keys(&self) -> Vec<String> {
        self.settings.saved.keys().map(|k| k.to_string()).collect()
    }

    pub fn insert_history(&mut self, cmd: TemplateBuilder) {
        self.settings.history.push(cmd);
        self.settings.history.truncate(self.settings.history_len);
    }

    pub fn get_history_entries(&self) -> Vec<String> {
        self.settings
            .history
            .iter()
            .enumerate()
            .map(|(index, builder)| {
                format!(
                    "{index}| {cmd}",
                    index = index,
                    cmd = builder.build_string()
                )
            })
            .collect()
    }

    pub fn get_history_entry(&self, index: usize) -> Option<&TemplateBuilder> {
        self.settings.history.get(index)
    }

    pub fn write(&mut self) {
        self.parent.insert_module(RUN_MODULE, &self.settings);
    }

    pub fn new(stored_settings: Box<dyn StoredSettings<SerializedSettings>>) -> Self {
        let settings: SerializedSettings = stored_settings.get_module(RUN_MODULE);

        Self {
            parent: stored_settings,
            settings,
        }
    }
}

impl SerializedSettings {
    pub fn default() -> Self {
        Self {
            history_len: 10,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {}
