use crate::endpoints::commands::{construct_curl_cmd};

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
        } => {
            let curl_cmd = construct_curl_endpoint(endpoint, method, data, base_url, headers);
            println!("{}", curl_cmd);
        }
    }
}
