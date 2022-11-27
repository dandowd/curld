use crate::endpoints::{
    endpoint_settings::EndpointSettings,
    run_cli::{RunInput, run},
    saved_cli::{SavedInput, saved}
};

#[derive(clap::Subcommand, Debug)]
pub enum Endpoints {
    Run(RunInput),

    Saved(SavedInput),
    List,
}

pub fn endpoints_match(endpoint_cmd: &Endpoints) {
    match endpoint_cmd {
        Endpoints::Run(input) => {
            let curl_cmd = run(input); 
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
