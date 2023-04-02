use crate::common::CurldCommand;
use crate::common::IO;
use crate::variables::builder::VariablesBuilder;
use std::collections::HashMap;

use super::settings::RunManager;
use super::utils::run_with_args;

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short, long)]
    pub id: Option<String>,

    // This is being used so clap doesn't try to interpret the curl args
    #[arg(raw = true)]
    pub user_args: Vec<String>,
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

impl RunCommand {
    pub fn run_match(
        run_cmd: &RunCommand,
        run_settings: &mut RunManager,
        variables_builder: &mut VariablesBuilder,
    ) {
        match run_cmd {
            RunCommand::Run(input) => {
                let RunInput { user_args, id } = input;

                let extracted_keys = variables_builder.extract(user_args);

                let user_values = RunCommand::prompt_for_variables(&extracted_keys);

                let curld_cmd = CurldCommand::new(user_args.to_owned(), user_values);

                let runnable_cmd = variables_builder.insert(&curld_cmd);
                let cmd_output = run_with_args(runnable_cmd);

                if let Some(id) = id {
                    run_settings.add_saved(id.to_owned(), curld_cmd.to_owned());
                }

                run_settings.insert_history(curld_cmd);
                IO::output(&cmd_output)
            }
            RunCommand::RunSaved { id } => {
                let curld_cmd = run_settings
                    .get_saved(id)
                    .expect("Could not find saved command");

                variables_builder.extract(&curld_cmd.user_args);

                let curl_output = run_with_args(variables_builder.insert(curld_cmd));

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
                            let output = run_with_args(variables_builder.insert(args));
                            IO::output(&output);
                        }
                        None => IO::output(&index.to_string()),
                    }
                }

                if input.list {
                    for history in run_settings.get_history_entries(variables_builder) {
                        IO::output(&history);
                    }
                }
            }
        }
    }

    fn prompt_for_variables(template_keys: &Vec<String>) -> HashMap<String, String> {
        let mut template_map: HashMap<String, String> = HashMap::new();
        RunCommand::loop_prompt(template_keys, &mut template_map);

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

#[cfg(test)]
mod tests {}
