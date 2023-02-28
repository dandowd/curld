use std::collections::HashMap;

use crate::run::settings::RunSettings;
use crate::settings::file::FileStorage;
use crate::settings::global_settings::GlobalSettings;

use super::TemplateBuilder;

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

pub fn run_match(run_cmd: &RunCommand) {
    let mut global_settings = GlobalSettings::new(FileStorage::new(None));
    let mut run_settings = RunSettings::new(&mut global_settings);

    match run_cmd {
        RunCommand::Run(input) => {
            let RunInput { cmd, id } = input;
            let mut template = TemplateBuilder::new(cmd.to_owned());
            let user_values = prompt_for_templates(&template.keys);
            template.insert_values(&user_values);

            let curl_output = run_with_args(template.cmd());

            if let Some(id) = id {
                run_settings.add_saved(id.to_owned(), template.to_owned());
            }

            run_settings.insert_history(template);
            run_settings.write();

            println!("{}", curl_output);
        }
        RunCommand::RunSaved { id } => {
            let template = run_settings
                .get_saved(id)
                .expect("Could not find saved command");

            let curl_output = run_with_args(template.cmd());

            print!("{}", curl_output)
        }
        RunCommand::List => {
            for id in run_settings.get_saved_keys() {
                println!("{}", id);
            }
        }
        RunCommand::History(input) => {
            if let Some(index) = input.run {
                let cmd = run_settings.get_history_entry(index);
                match cmd {
                    Some(args) => {
                        let output = run_with_args(args.cmd());
                        println!("{}", output);
                    }
                    None => println!("No history at index {}", index),
                }
            }

            if input.list {
                for history in run_settings.get_history_entries() {
                    println!("{:?}", history);
                }
            }
        }
    }

    global_settings.write();
}

fn prompt_for_templates(template_keys: &Vec<String>) -> HashMap<String, String> {
    let mut template_map: HashMap<String, String> = HashMap::new();
    loop_prompt(template_keys, &mut template_map);
    println!();

    template_map
}

fn loop_prompt(keys: &Vec<String>, map: &mut HashMap<String, String>) {
    for key in keys {
        if map.contains_key(key) {
            continue;
        }
        let value = prompt_for_key(key);
        map.insert(key.to_owned(), value);
    }
}

fn prompt_for_key(key: &str) -> String {
    use std::io::{stdin, stdout, Write};
    print!("Enter value for {}: ", key);
    stdout().flush().expect("unable to flush stdout");

    let mut output = String::new();
    stdin().read_line(&mut output).expect("No input");
    //read_line will include the new line char, so output needs to be trimmed
    output.trim().to_string()
}
