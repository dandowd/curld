use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{common::CurldCommand, settings::traits::StoredSettings};

use super::mutators::WorkspaceMutator;

pub static WORKSPACE_MODULE: &str = "workspace";

pub struct WorkspaceManager {
    workspace_settings: WorkspaceSettings,
}

#[derive(Deserialize, Serialize, Default)]
pub struct WorkspaceSettings {
    pub current_workspace: String,
    workspaces: HashMap<String, Workspace>,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Workspace {
    pub name: String,
    pub variables: HashMap<String, String>,
    pub commands: Vec<CurldCommand>,
}

impl WorkspaceManager {
    pub fn new(stored_settings: &dyn StoredSettings<WorkspaceSettings>) -> Self {
        let workspace_settings = stored_settings
            .get_module(WORKSPACE_MODULE)
            .unwrap_or_default();

        Self { workspace_settings }
    }

    pub fn change_workspace(&mut self, workspace_name: &str) {
        if let None = self.workspace_settings.workspaces.get(workspace_name) {
            self.workspace_settings
                .workspaces
                .insert(workspace_name.to_string(), Workspace::default());
        }

        self.workspace_settings.current_workspace = workspace_name.to_string();
    }

    pub fn get_current_workspace(&self) -> &Workspace {
        self.workspace_settings
            .workspaces
            .get(&self.workspace_settings.current_workspace)
            .expect(
                "No workspace found, try changing to the workspace again to create a default one.",
            )
    }

    pub fn get_workspace_mutator(&self) -> WorkspaceMutator {
        WorkspaceMutator::new(self.get_current_workspace())
    }
}

#[cfg(test)]
mod tests {}
