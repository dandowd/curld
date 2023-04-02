use crate::common::IO;

use super::settings::WorkspacesManager;

#[derive(clap::Subcommand, Debug)]
pub enum WorkspacesCommand {
    List,
    Create { name: String },
    Use { name: String },
    SetVariable { key: String, value: String },
}

impl WorkspacesCommand {
    pub fn cli_match(command: &WorkspacesCommand, workspaces_manager: &mut WorkspacesManager) {
        match command {
            WorkspacesCommand::List => {
                let list = workspaces_manager.get_workspaces_names();
                IO::output(&list.join("\n"));
            }
            WorkspacesCommand::Create { name } => {
                workspaces_manager.change_workspace(&name);
                IO::output(&format!("Workspace created: {}", name));
            }
            WorkspacesCommand::Use { name } => {
                workspaces_manager.change_workspace(&name);
                IO::output(&format!("Workspace changed to {}", name));
            }
            WorkspacesCommand::SetVariable { key, value } => {
                workspaces_manager.set_variable(key, value);
                IO::output(&format!("Variable set: {}={}", key, value));
            }
        }
    }
}

#[cfg(test)]
mod tests {}
