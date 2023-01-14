use std::collections::HashMap;

use super::TemplateBuilder;

use super::endpoint_settings::EndpointSettings;
use super::run::run_with_args;

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short, long)]
    pub id: Option<String>,

    #[arg(short, long)]
    pub endpoint: Option<String>,

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
pub enum Endpoints {
    Run(RunInput),
    History(HistoryInput),
    RunSaved { id: String },
    List,
}

pub fn endpoints_match(endpoint_cmd: &Endpoints) {
    match endpoint_cmd {
        Endpoints::Run(input) => {
            let RunInput {
                cmd,
                endpoint: _,
                id,
            } = input;
            let mut template = TemplateBuilder::new(cmd.to_owned());
            let user_values = prompt_for_templates(&template.keys);
            template.insert_values(&user_values);

            let curl_output = run_with_args(template.cmd());

            let mut endpoint_settings = EndpointSettings::get();

            if let Some(id) = id {
                endpoint_settings.add_saved(id.to_owned(), template.to_owned());
            }

            endpoint_settings.insert_history(template);
            endpoint_settings.write();

            println!("{}", curl_output);
        }
        Endpoints::RunSaved { id } => {
            let settings = EndpointSettings::get();
            let template = settings
                .get_saved(id)
                .expect("Could not find saved command");

            let curl_output = run_with_args(template.cmd());

            print!("{}", curl_output)
        }
        Endpoints::List => {
            let settings = EndpointSettings::get();
            for id in settings.get_saved_keys() {
                println!("{}", id);
            }
        }
        Endpoints::History(input) => {
            let settings = EndpointSettings::get();

            if let Some(index) = input.run {
                let cmd = settings.get_history_entry(index);
                match cmd {
                    Some(args) => {
                        let output = run_with_args(args.cmd());
                        println!("{}", output);
                    }
                    None => println!("No history at index {}", index),
                }
            }

            if input.list {
                for history in settings.get_history_entries() {
                    println!("{:?}", history);
                }
            }
        }
    }
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
