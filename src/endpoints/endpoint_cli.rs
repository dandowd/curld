use std::collections::HashMap;

use crate::endpoints::{
    endpoint_settings::get_endpoint_settings,
    run::{run, run_with_args},
    saved::{saved, SavedInput},
};

use crate::templates::parse::{
    extract_template_names,
    insert_template_values,
    insert_template_values_vec
};

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short = 'X', long, default_value = "GET", required = false)]
    pub method: String,

    #[arg(short, long, default_value = "", required = false)]
    pub base_url: String,

    #[arg(short, long, default_value = "", required = false)]
    pub data: String,

    #[arg(short = 'H', long, required = false)]
    pub headers: Vec<String>,

    #[arg(short, long)]
    pub id: Option<String>,

    pub endpoint: String,
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
    Saved(SavedInput),
    History(HistoryInput),
    List,
}

pub fn endpoints_match(endpoint_cmd: &Endpoints) {
    match endpoint_cmd {
        Endpoints::Run(input) => {
            let RunInput {
                endpoint,
                base_url,
                data,
                headers,
                method,
                id,
            } = input;
            let template_keys = get_template_keys(endpoint, data, base_url, headers);
            let user_values = prompt_for_templates(template_keys);

            let endpoint = insert_template_values(endpoint, &user_values);
            let data_str = insert_template_values(data, &user_values);
            let base_url_str = insert_template_values(base_url, &user_values);
            let header_str = insert_template_values_vec(headers, &user_values);

            let curl_output = run(&endpoint, method, &data_str, &base_url_str, &header_str, id);

            println!("{}", curl_output);
        }
        Endpoints::Saved(input) => {
            let curl_cmd = saved(input);
            println!("{}", curl_cmd);
        }
        Endpoints::List => {
            let (settings, _) = get_endpoint_settings();
            for id in settings.get_saved_keys() {
                println!("{}", id);
            }
        }
        Endpoints::History(input) => {
            let (settings, _) = get_endpoint_settings();

            if let Some(index) = input.run {
                let cmd_args = settings.get_history_entry(index);
                match cmd_args {
                    Some(args) => {
                        let curl_arg_vec: Vec<String> =
                            args.split(' ').map(|item| item.to_string()).collect();
                        let output = run_with_args(curl_arg_vec);
                        println!("{}", output);
                    }
                    None => println!("No history at index {}", index),
                }
            }

            if input.list {
                for history in settings.get_history_entries() {
                    println!("{}", history);
                }
            }
        }
    }
}

pub struct Templates {
    pub endpoint: Vec<String>,
    pub data: Vec<String>,
    pub base_url: Vec<String>,
    pub headers: Vec<String>,
}

pub fn get_template_keys(
    endpoint: &str,
    data: &str,
    base_url: &str,
    headers: &Vec<String>,
) -> Templates {
    let endpoint_templates = extract_template_names(endpoint).unwrap();
    let data_templates = extract_template_names(data).unwrap();
    let base_url_templates = extract_template_names(base_url).unwrap();
    let header_templates = {
        let mut values = Vec::new();
        for header in headers {
            let mut template_names = extract_template_names(header).unwrap();
            values.append(&mut template_names);
        }

        values
    };

    Templates {
        endpoint: endpoint_templates,
        data: data_templates,
        base_url: base_url_templates,
        headers: header_templates,
    }
}

fn prompt_for_templates(template_keys: Templates) -> HashMap<String, String> {
    let Templates {
        endpoint,
        data,
        base_url,
        headers,
    } = template_keys;
    let mut template_map: HashMap<String, String> = HashMap::new();
    println!("Endpoint Templates");
    loop_prompt(endpoint, &mut template_map);
    println!();

    println!("Data Templates");
    loop_prompt(data, &mut template_map);
    println!();

    println!("Base URL Templates");
    loop_prompt(base_url, &mut template_map);
    println!();

    println!("Header Templates");
    loop_prompt(headers, &mut template_map);
    println!();

    template_map
}

fn loop_prompt(template: Vec<String>, map: &mut HashMap<String, String>) {
    for key in template {
        if map.contains_key(&key) {
            continue;
        }
        let value = prompt_for_key(&key);
        map.insert(key, value);
    }
}

fn prompt_for_key(key: &String) -> String {
    use std::io::{stdin, stdout, Write};
    print!("Enter value for {}: ", key);
    let _ = stdout().flush();

    let mut output = String::new();
    stdin().read_line(&mut output).expect("No input");
    //read_line will include the new line char, so output needs to be trimmed
    output.trim().to_string()
}
