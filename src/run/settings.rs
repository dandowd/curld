use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default};

use crate::{
    common::CurldCommand, settings::traits::StoredSettings, variables::builder::VariablesBuilder,
};

pub static RUN_MODULE: &str = "run";

pub struct RunManager<'a> {
    parent: &'a mut dyn StoredSettings<RunSettings>,

    settings: RunSettings,
}

#[derive(Deserialize, Serialize, Default)]
pub struct RunSettings {
    #[serde(default)]
    saved: HashMap<String, CurldCommand>,

    #[serde(default)]
    history_len: usize,

    #[serde(default)]
    history: Vec<CurldCommand>,
}

impl<'a> RunManager<'a> {
    pub fn add_saved(&mut self, id: String, history: CurldCommand) {
        self.settings.saved.insert(id, history);
    }

    pub fn get_saved(&self, id: &String) -> Option<&CurldCommand> {
        self.settings.saved.get(id)
    }

    pub fn get_saved_keys(&self) -> Vec<String> {
        self.settings.saved.keys().map(|k| k.to_string()).collect()
    }

    pub fn insert_history(&mut self, cmd: CurldCommand) {
        self.settings.history.push(cmd);
        self.settings.history.truncate(self.settings.history_len);

        self.save_to_parent();
    }

    pub fn get_history_entries(&self, builder: &VariablesBuilder) -> Vec<String> {
        self.settings
            .history
            .iter()
            .enumerate()
            .map(|(index, curld)| {
                format!(
                    "{index}| {cmd}",
                    index = index,
                    cmd = builder.to_string(curld)
                )
            })
            .collect()
    }

    pub fn get_history_entry(&self, index: usize) -> Option<&CurldCommand> {
        self.settings.history.get(index)
    }

    fn save_to_parent(&mut self) {
        self.parent.insert_module(RUN_MODULE, &self.settings);
    }

    pub fn new<'b: 'a>(stored_settings: &'b mut dyn StoredSettings<RunSettings>) -> Self {
        let settings: RunSettings = stored_settings
            .get_module(RUN_MODULE)
            .unwrap_or_else(RunSettings::default);

        Self {
            parent: stored_settings,
            settings,
        }
    }
}

impl RunSettings {
    pub fn default() -> Self {
        Self {
            history_len: 10,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn should_return_history_list() {}
}
