use std::{cell::RefCell, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::settings::traits::StoredSettings;

static COMMAND_MODULE: &str = "command";

pub struct CommandManager<'a> {
    pub commands: Commands,
    pub stored_settings: &'a RefCell<dyn StoredSettings<Commands>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Commands {
    pub commands_map: HashMap<String, CommandSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandSettings {
    pub command: String,
    pub saved_args: Vec<String>,
}

impl<'a> CommandManager<'a> {
    pub fn new(stored_settings: &'a RefCell<dyn StoredSettings<Commands>>) -> Self {
        let commands = stored_settings.borrow().get_module(COMMAND_MODULE);

        Self {
            commands: commands.unwrap_or_default(),
            stored_settings,
        }
    }

    pub fn get_command(&self, cmd: &str) -> CommandSettings {
        match self.commands.commands_map.get(cmd) {
            Some(cmd) => cmd.clone(),
            None => CommandSettings {
                command: String::from(cmd),
                saved_args: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::settings::traits::MockStoredSettings;

    fn default_stored_settings() -> RefCell<MockStoredSettings<Commands>> {
        let mut stored_settings = MockStoredSettings::new();
        stored_settings.expect_get_module().returning(|_| None);

        RefCell::new(stored_settings)
    }

    #[test]
    fn new_should_return_empty_hash_map() {
        let stored_settings = default_stored_settings();

        let command_manager = CommandManager::new(&stored_settings);

        assert_eq!(command_manager.commands.commands_map.len(), 0);
    }

    #[test]
    fn get_command_should_return_default_command() {
        let stored_settings = default_stored_settings();

        let command_manager = CommandManager::new(&stored_settings);

        let command = command_manager.get_command("test");

        assert_eq!(command.command, "test");
        assert_eq!(command.saved_args.len(), 0);
    }
}
