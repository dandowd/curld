use crate::endpoints::{
    commands::construct_curl_cmd,
    endpoint_settings::{EndpointSettings, SavedEndpoint},
};

#[derive(clap::Subcommand, Debug)]
pub enum Endpoints {
    Run {
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
    },

    Saved {
        id: String,
    },
}

pub fn endpoints_match(endpoint_cmd: &Endpoints) {
    match endpoint_cmd {
        Endpoints::Run {
            endpoint,
            method,
            base_url,
            data,
            headers,
            id,
        } => {
            let curl_cmd = construct_curl_cmd(endpoint, method, data, base_url, headers);
            if let Some(id_str) = id {
                let global_settings = crate::global_settings::get();
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
                global_settings.write();
            }
            println!("{}", curl_cmd);
        }
        Endpoints::Saved { id } => {
            let global_settings = crate::global_settings::get();
            let settings: EndpointSettings =
                global_settings.get_module(super::endpoint_settings::ENDPOINT_MODULE);
            let SavedEndpoint { endpoint, method, data, base_url, headers } = settings.get_saved(id).expect("Unable to find saved endpoint");

            let curl_cmd = construct_curl_cmd(endpoint, method, data, base_url, headers);
            
            println!("{}", curl_cmd);
        }
    }
}
