pub enum WorkspacesCommand {
    List,
    Create { name: String },
    Use { name: String },
}

pub struct WorkspacesCli {}

impl WorkspacesCli {
    pub fn run_match(command: &WorkspacesCommand) {
        match command {
            WorkspacesCommand::List => {}
            WorkspacesCommand::Create { name } => {}
            WorkspacesCommand::Use { name } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
