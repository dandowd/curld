use super::mutators::WorkspaceMutator;
use crate::{common::CurldCommand, settings::traits::StoredSettings};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

pub static WORKSPACE_MODULE: &str = "workspace";

pub struct WorkspacesManager<'a> {
    stored_settings: &'a RefCell<dyn StoredSettings<WorkspaceSettings>>,
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

impl<'a> WorkspacesManager<'a> {
    pub fn new(stored_settings: &'a RefCell<dyn StoredSettings<WorkspaceSettings>>) -> Self {
        let mut workspace_settings = stored_settings
            .borrow_mut()
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

        stored_settings
            .borrow_mut()
            .insert_module(WORKSPACE_MODULE, &workspace_settings);

        Self {
            stored_settings,
            workspace_settings,
        }
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

        self.stored_settings
            .borrow_mut()
            .insert_module(WORKSPACE_MODULE, &self.workspace_settings);
    }

    pub fn get_current_workspace(&self) -> &Workspace {
        self.workspace_settings
            .workspaces
            .get(&self.workspace_settings.current_workspace)
            .expect(
                "No workspace found, try changing to the workspace again to create a default one.",
            )
    }

    pub fn get_workspaces_names(&self) -> Vec<String> {
        self.workspace_settings
            .workspaces
            .keys()
            .map(|key| key.to_string())
            .collect()
    }

    pub fn set_variable(&mut self, key: &str, value: &str) {
        let mut workspace = self.get_current_workspace().clone();

        workspace
            .variables
            .insert(key.to_string(), value.to_string());

        self.workspace_settings
            .workspaces
            .insert(workspace.name.clone(), workspace);

        self.stored_settings
            .borrow_mut()
            .insert_module(WORKSPACE_MODULE, &self.workspace_settings);
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
    fn change_workspace_should_return_workspace_even_when_it_does_not_exist() {
        let mut stored_settings = RefCell::new(MockStoredSettings::new());
        stored_settings
            .borrow_mut()
            .expect_get_module()
            .returning(|_| None);
        stored_settings
            .borrow_mut()
            .expect_insert_module()
            .returning(|_, _| ());

        let mut manager = WorkspacesManager::new(&mut stored_settings);
        manager.change_workspace("test");

        let current_workspace = manager.get_current_workspace();

        assert_eq!(current_workspace.name, "test");
    }

    #[test]
    fn new_should_create_default_workspace_when_none_exist() {
        let mut stored_settings = RefCell::new(MockStoredSettings::new());
        stored_settings
            .borrow_mut()
            .expect_get_module()
            .returning(|_| None);
        stored_settings
            .borrow_mut()
            .expect_insert_module()
            .returning(|_, _| ());

        let manager = WorkspacesManager::new(&mut stored_settings);

        assert_eq!(manager.workspace_settings.workspaces.len(), 1);
    }
}
