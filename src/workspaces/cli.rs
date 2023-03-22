use crate::common::IO;

use super::settings::WorkspacesManager;

#[derive(clap::Subcommand, Debug)]
pub enum WorkspacesCommand {
    List,
    Create { name: String },
    Use { name: String },
}

pub struct WorkspacesCli {}

impl WorkspacesCli {
    pub fn run_match(command: &WorkspacesCommand, workspaces_manager: &mut WorkspacesManager) {
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
        }
    }
}

#[cfg(test)]
mod tests {}
