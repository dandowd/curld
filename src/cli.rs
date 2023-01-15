use clap::Parser;

use crate::run::run_cli::{run_match, Run};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    #[command(flatten)]
    Run(Run),
}

pub fn run() {
    let input = Args::parse();
    match &input.command {
        Commands::Run(variants) => run_match(variants),
    }
}
