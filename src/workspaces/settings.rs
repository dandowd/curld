use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::settings::traits::StoredSettings;

#[derive(Deserialize, Serialize, Default)]
pub struct WorkspaceSettings {
    pub current_workspace: String,
    workspaces: HashMap<String, Workspace>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Workspace {
    pub base_url: String,
    pub headers: Vec<String>,
}

impl WorkspaceSettings {
    fn new(stored_settings: Box<dyn StoredSettings<WorkspaceSettings>>) -> Self {
        Self {
            current_workspace: String::from("default"),
            workspaces: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {}
