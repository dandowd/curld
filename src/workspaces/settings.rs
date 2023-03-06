use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::settings::traits::StoredSettings;

pub static WORKSPACE_MODULE: &str = "workspace";

pub struct WorkspaceManager<'a> {
    stored_settings: &'a dyn StoredSettings<WorkspaceSettings>,

    workspace_settings: WorkspaceSettings,
}

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

impl<'a> WorkspaceManager<'a> {
    fn new<'b: 'a>(stored_settings: &'b dyn StoredSettings<WorkspaceSettings>) -> Self {
        let workspace_settings = stored_settings
            .get_module(WORKSPACE_MODULE)
            .unwrap_or_else(Default::default);

        Self {
            stored_settings,
            workspace_settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::traits::MockStoredSettings;

    use super::*;

    fn should_use_workspace_settings() {
        let stored_settings = MockStoredSettings::new();
        let manager = WorkspaceManager::new(&stored_settings);
    }
}
