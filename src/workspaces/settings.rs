use super::mutators::WorkspaceMutator;
use crate::{common::CurldCommand, settings::traits::StoredSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        let mut workspace_settings = stored_settings
            .get_module(WORKSPACE_MODULE)
            .unwrap_or_default();

        if workspace_settings.workspaces.is_empty() {
            let default_name = "default".to_string();

            let default_workspace = Workspace {
                name: String::from(&default_name),
                ..Default::default()
            };

            workspace_settings
                .workspaces
                .insert(String::from(&default_name), default_workspace);

            workspace_settings.current_workspace = String::from(&default_name);
        }

        Self { workspace_settings }
    }

    pub fn change_workspace(&mut self, workspace_name: &str) {
        if let None = self.workspace_settings.workspaces.get(workspace_name) {
            let created_workspace = Workspace {
                name: workspace_name.to_string(),
                ..Default::default()
            };

            self.workspace_settings
                .workspaces
                .insert(workspace_name.to_string(), created_workspace);
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
mod tests {
    use super::*;
    use crate::settings::traits::MockStoredSettings;

    #[test]
    fn it_should_return_workspace_even_when_it_does_not_exist() {
        let mut stored_settings = MockStoredSettings::new();
        stored_settings.expect_get_module().returning(|_| None);

        let mut manager = WorkspaceManager::new(&stored_settings);
        manager.change_workspace("test");

        let current_workspace = manager.get_current_workspace();

        assert_eq!(current_workspace.name, "test");
    }
}
