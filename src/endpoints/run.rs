use super::{endpoint_settings::get_endpoint_settings, utils::construct_curl_args};
use crate::endpoints::endpoint_settings::SavedEndpoint;
use std::process::Command;

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

pub fn run(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
    id: &Option<String>,
) -> String {
    let curl_args = construct_curl_args(endpoint, method, data, base_url, headers);

    let (mut endpoint_settings, global_settings) = get_endpoint_settings();
    endpoint_settings.insert_history(&curl_args.join(" "));
    if let Some(id_str) = id {
        let saved_command = SavedEndpoint {
            endpoint: String::from(endpoint),
            method: String::from(method),
            headers: headers.to_owned(),
            base_url: base_url.to_owned(),
            data: data.to_owned(),
        };

        endpoint_settings.add_saved(String::from(id_str), saved_command);
    }

    global_settings.write();

    run_with_args(curl_args)
}

pub fn run_with_args(curl_args: Vec<String>) -> String {
    let output = Command::new("curl").args(curl_args).output();
    match output {
        Ok(cmd_out) => String::from_utf8(cmd_out.stdout).unwrap(),
        Err(msg) => panic!("{:?}", msg),
    }
}
