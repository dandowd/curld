use clap::Parser;

use crate::{
    config::Config,
    run::cli::{RunCli, RunCommand},
    settings::{file::FileStorage, global_settings::GlobalSettings},
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

    let variable_mutators = Config::get_mutators();

    match &input.command {
        Commands::Run(variants) => {
            RunCli::run_match(variants, &mut global_settings, &variable_mutators)
        }
    }

    global_settings.write();
}
