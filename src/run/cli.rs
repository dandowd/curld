use crate::common::IO;
use crate::settings::traits::StoredSettings;
use crate::variables::builder::VariablesBuilder;
use crate::variables::mutators::VariableMutators;
use std::collections::HashMap;

use super::settings::RunManager;
use super::settings::RunSettings;
use super::utils::run_with_args;

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short, long)]
    pub id: Option<String>,

    #[arg(raw = true)]
    pub cmd: Vec<String>,
}

#[derive(clap::Args, Debug)]
pub struct HistoryInput {
    #[arg(short, long, default_value = "false")]
    pub list: bool,

    #[arg(short, long)]
    pub run: Option<usize>,
}

#[derive(clap::Subcommand, Debug)]
pub enum RunCommand {
    Run(RunInput),
    History(HistoryInput),
    RunSaved { id: String },
    List,
}

pub struct RunCli {}

impl RunCli {
    pub fn run_match(
        run_cmd: &RunCommand,
        stored_settings: &mut dyn StoredSettings<RunSettings>,
        variables_mutators: &VariableMutators,
    ) {
        let mut run_settings = RunManager::new(stored_settings);
        let mut builder = VariablesBuilder::new(variables_mutators);

        match run_cmd {
            RunCommand::Run(input) => {
                let RunInput { cmd, id } = input;
                builder.set_original_args(cmd).extract_keys();

                let user_values = RunCli::prompt_for_templates(&builder.keys);

                let runnable_cmd = builder.set_value_map(&user_values).cmd();
                let curl_output = run_with_args(runnable_cmd);

                if let Some(id) = id {
                    run_settings.add_saved(id.to_owned(), builder.build_curld_cmd());
                }

                run_settings.insert_history(builder.build_curld_cmd());
                IO::output(&curl_output)
            }
            RunCommand::RunSaved { id } => {
                let curld = run_settings
                    .get_saved(id)
                    .expect("Could not find saved command");

                builder.fill(curld);

                let curl_output = run_with_args(builder.cmd());

                IO::output(&curl_output)
            }
            RunCommand::List => {
                for id in run_settings.get_saved_keys() {
                    IO::output(&id);
                }
            }
            RunCommand::History(input) => {
                if let Some(index) = input.run {
                    let cmd = run_settings.get_history_entry(index);
                    match cmd {
                        Some(args) => {
                            let output = run_with_args(builder.fill(args).cmd());
                            IO::output(&output);
                        }
                        None => IO::output(&index.to_string()),
                    }
                }

                if input.list {
                    for history in
                        run_settings.get_history_entries(VariablesBuilder::new(variables_mutators))
                    {
                        IO::output(&history);
                    }
                }
            }
        }
    }

    fn prompt_for_templates(template_keys: &Vec<String>) -> HashMap<String, String> {
        let mut template_map: HashMap<String, String> = HashMap::new();
        RunCli::loop_prompt(template_keys, &mut template_map);

        template_map
    }

    fn loop_prompt(keys: &Vec<String>, map: &mut HashMap<String, String>) {
        for key in keys {
            if map.contains_key(key) {
                continue;
            }
            let value = IO::prompt(&format!("Enter value for {}: ", key));
            map.insert(key.to_owned(), value);
        }
    }
}
