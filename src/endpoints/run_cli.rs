use crate::endpoints::{
    utils::construct_curl_cmd,
    endpoint_settings::{ SavedEndpoint, EndpointSettings }
};

#[derive(clap::Args, Debug)]
pub struct RunInput {
    #[arg(short)]
    endpoint: String,

    #[arg(short = 'X', long)]
    method: String,

    #[arg(short, long)]
    base_url: Option<String>,

    #[arg(short, long)]
    data: Option<String>,

    #[arg(short = 'H', long)]
    headers: Option<Vec<String>>,

    #[arg(short, long)]
    id: Option<String>,
}

pub fn run(input: &RunInput) -> String {
    let RunInput {
        endpoint,
        method,
        data,
        base_url,
        headers,
        id,
    } = input;
    let curl_cmd = construct_curl_cmd(endpoint, method, data, base_url, headers);
    if let Some(id_str) = id {
        let mut global_settings = crate::global_settings::get();
        let mut settings: EndpointSettings =
            global_settings.get_module(super::endpoint_settings::ENDPOINT_MODULE);

        let saved_command = SavedEndpoint {
            endpoint: String::from(endpoint),
            method: String::from(method),
            headers: headers.to_owned(),
            base_url: base_url.to_owned(),
            data: data.to_owned(),
        };

        settings.add_saved(String::from(id_str), saved_command);
        global_settings.insert_module(super::endpoint_settings::ENDPOINT_MODULE, settings);
        global_settings.write();
    }

    curl_cmd
}
