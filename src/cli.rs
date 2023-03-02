use clap::Parser;

use crate::{
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

    match &input.command {
        Commands::Run(variants) => {
            let cli = RunCli::new(prompt, output);
            cli.run_match(variants, &mut global_settings)
        }
    }

    global_settings.write();
}

fn output(message: &str) {
    print!("{}", message);
}

fn prompt(message: &str) -> String {
    use std::io::{stdin, stdout, Write};
    print!("{}", message);
    stdout().flush().expect("unable to flush stdout");

    let mut output = String::new();
    stdin().read_line(&mut output).expect("No input");
    //read_line will include the new line char, so output needs to be trimmed
    output.trim().to_string()
}
