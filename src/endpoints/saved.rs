use crate::endpoints::{
    endpoint_settings::{EndpointSettings, SavedEndpoint},
    utils::construct_curl_cmd,
};

#[derive(clap::Args, Debug)]
pub struct SavedInput {
    #[arg(short, long)]
    id: String,
}

pub fn saved(input: &SavedInput) -> String {
    let SavedInput { id } = input;
    let global_settings = crate::global_settings::get_global_settings();
    let settings: EndpointSettings =
        global_settings.get_module(super::endpoint_settings::ENDPOINT_MODULE);
    let SavedEndpoint {
        endpoint,
        method,
        data,
        base_url,
        headers,
    } = settings
        .get_saved(id)
        .expect("Unable to find saved endpoint");

    construct_curl_cmd(endpoint, method, data, base_url, headers)
}
