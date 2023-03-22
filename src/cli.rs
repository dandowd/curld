use clap::Parser;

use crate::{
    run::{
        cli::{RunCli, RunCommand},
        settings::RunManager,
    },
    settings::{file::FileStorage, global_settings::GlobalSettings},
    variables::builder::VariablesBuilder,
    workspaces::settings::WorkspaceManager,
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
}

pub fn run() {
    let input = Args::parse();
    let mut global_settings = GlobalSettings::new(FileStorage::new(None));
    let mut variable_builder = VariablesBuilder::new();

    let workspace_settings = WorkspaceManager::new(&mut global_settings);
    let workspace_mutator = workspace_settings.get_workspace_mutator();

    variable_builder.add_extractor(&workspace_mutator);
    variable_builder.add_inserter(&workspace_mutator);

    let mut run_settings = RunManager::new(&mut global_settings);
    let run_mutators = run_settings.get_mutators();

    variable_builder.add_extractor(&run_mutators);
    variable_builder.add_inserter(&run_mutators);

    match &input.command {
        Commands::Run(variants) => {
            RunCli::run_match(variants, &mut run_settings, &mut variable_builder)
        }
    }

    global_settings.write();
}
