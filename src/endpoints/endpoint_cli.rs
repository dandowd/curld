use std::collections::HashMap;

use crate::endpoints::{
    endpoint_settings::EndpointSettings,
    saved::{saved, SavedInput},
    run::run,
    utils::{extract_template_names, insert_template_values, insert_template_values_vec},
};

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short = 'X', long)]
    pub method: Option<String>,

    #[arg(short, long)]
    pub base_url: Option<String>,

    #[arg(short, long)]
    pub data: Option<String>,

    #[arg(short = 'H', long)]
    pub headers: Option<Vec<String>>,

    #[arg(short, long)]
    pub id: Option<String>,

    pub endpoint: String,
}

#[derive(clap::Subcommand, Debug)]
pub enum Endpoints {
    Run(RunInput),

    Saved(SavedInput),
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
            let header_str = headers.clone().unwrap_or(Vec::new());
            let base_url_str = base_url.clone().unwrap_or("".to_string());
            let data_str = data.clone().unwrap_or("".to_string());
            let method_str = method.clone().unwrap_or("GET".to_string());

            let template_keys = get_template_keys(&endpoint, &data_str, &base_url_str, &header_str);
            let user_templates = prompt_for_templates(template_keys);

            let endpoint = insert_template_values(&endpoint, &user_templates);
            let data_str = insert_template_values(&data_str, &user_templates);
            let base_url_str = insert_template_values(&base_url_str, &user_templates);
            let header_str = insert_template_values_vec(&header_str, &user_templates);

            let curl_cmd = run(
                &endpoint,
                &method_str,
                &data_str,
                &base_url_str,
                &header_str,
                &id,
            );
            println!("{}", curl_cmd);
        }
        Endpoints::Saved(input) => {
            let curl_cmd = saved(input);
            println!("{}", curl_cmd);
        }
        Endpoints::List => {
            let global_settings = crate::global_settings::get();
            let settings: EndpointSettings =
                global_settings.get_module(super::endpoint_settings::ENDPOINT_MODULE);
            for id in settings.get_saved_keys() {
                println!("{}", id);
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
    endpoint: &String,
    data: &String,
    base_url: &String,
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
    println!("");

    println!("Data Templates");
    loop_prompt(data, &mut template_map);
    println!("");

    println!("Base URL Templates");
    loop_prompt(base_url, &mut template_map);
    println!("");

    println!("Header Templates");
    loop_prompt(headers, &mut template_map);
    println!("");

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
