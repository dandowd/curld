use clap::Parser;

use crate::endpoints::endpoint_cli::{endpoints_match, Endpoints};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    #[command(flatten)]
    Endpoints(Endpoints),
}

pub fn run() {
    let input = Args::parse();
    match &input.command {
        Commands::Endpoints(variants) => endpoints_match(variants),
    }
}
