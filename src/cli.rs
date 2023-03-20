use clap::Parser;

use crate::{
    run::{
        cli::{RunCli, RunCommand},
        mutators::RunMutators,
    },
    settings::{file::FileStorage, global_settings::GlobalSettings},
    variables::mutators::VariableMutators,
    workspaces::mutators::WorkspaceMutators,
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
    let mut variable_mutators = VariableMutators::new();

    variable_mutators.register_inserters(WorkspaceMutators::get_inserters());
    variable_mutators.register_extractors(WorkspaceMutators::get_extractors());

    variable_mutators.register_inserters(RunMutators::get_inserters());
    variable_mutators.register_extractors(RunMutators::get_extractors());

    match &input.command {
        Commands::Run(variants) => {
            RunCli::run_match(variants, &mut global_settings, &variable_mutators)
        }
    }

    global_settings.write();
}
