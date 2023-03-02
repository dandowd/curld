use crate::settings::traits::StoredSettings;
use crate::variables::variables_builder::VariablesBuilder;
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

type Prompt = fn(message: &str) -> String;
type Output = fn(message: &str);

pub struct RunCli {
    prompt: Prompt,
    output: Output,
}

impl RunCli {
    pub fn new(prompt: Prompt, output: Output) -> Self {
        Self { prompt, output }
    }

    pub fn run_match(
        &self,
        run_cmd: &RunCommand,
        stored_settings: &mut dyn StoredSettings<RunSettings>,
    ) {
        let mut run_settings = RunManager::new(stored_settings);

        match run_cmd {
            RunCommand::Run(input) => {
                let RunInput { cmd, id } = input;
                let mut template = VariablesBuilder::new(cmd.to_owned());
                let user_values = self.prompt_for_templates(&template.keys);
                template.insert_values(&user_values);

                let curl_output = run_with_args(template.cmd());

                if let Some(id) = id {
                    run_settings.add_saved(id.to_owned(), template.to_owned());
                }

                run_settings.insert_history(template);
                (self.output)(&curl_output)
            }
            RunCommand::RunSaved { id } => {
                let template = run_settings
                    .get_saved(id)
                    .expect("Could not find saved command");

                let curl_output = run_with_args(template.cmd());

                (self.output)(&curl_output)
            }
            RunCommand::List => {
                for id in run_settings.get_saved_keys() {
                    (self.output)(&id);
                }
            }
            RunCommand::History(input) => {
                if let Some(index) = input.run {
                    let cmd = run_settings.get_history_entry(index);
                    match cmd {
                        Some(args) => {
                            let output = run_with_args(args.cmd());
                            (self.output)(&output);
                        }
                        None => (self.output)(&index.to_string()),
                    }
                }

                if input.list {
                    for history in run_settings.get_history_entries() {
                        (self.output)(&history);
                    }
                }
            }
        }
    }

    fn prompt_for_templates(&self, template_keys: &Vec<String>) -> HashMap<String, String> {
        let mut template_map: HashMap<String, String> = HashMap::new();
        self.loop_prompt(template_keys, &mut template_map);

        template_map
    }

    fn loop_prompt(&self, keys: &Vec<String>, map: &mut HashMap<String, String>) {
        for key in keys {
            if map.contains_key(key) {
                continue;
            }
            let value = (self.prompt)(&format!("Enter value for {}: ", key));
            map.insert(key.to_owned(), value);
        }
    }
}
