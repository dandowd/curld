use std::cell::RefCell;

use clap::Parser;

use crate::{
    run::{cli::RunCommand, settings::RunManager},
    settings::{file::FileStorage, global_settings::GlobalSettings},
    variables::builder::VariablesBuilder,
    workspaces::{cli::WorkspacesCommand, settings::WorkspacesManager},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    #[command(flatten)]
    Run(RunCommand),

    #[command(subcommand)]
    Workspaces(WorkspacesCommand),
}

pub fn run() {
    let input = Args::parse();
    let global_settings = RefCell::new(GlobalSettings::new(FileStorage::new(None)));
    let mut variable_builder = VariablesBuilder::new();

    let mut workspace_settings = WorkspacesManager::new(&global_settings);
    let workspace_mutator = workspace_settings.get_workspace_mutator();

    variable_builder.add_inserter(&workspace_mutator);

    let mut run_settings = RunManager::new(&global_settings);
    let run_mutators = run_settings.get_mutators();

    variable_builder.add_extractor(&run_mutators);
    variable_builder.add_inserter(&run_mutators);

    match &input.command {
        Commands::Run(variants) => {
            RunCommand::run_match(variants, &mut run_settings, &mut variable_builder)
        }
        Commands::Workspaces(variants) => {
            WorkspacesCommand::cli_match(variants, &mut workspace_settings)
        }
    }

    global_settings.borrow_mut().write();
}
