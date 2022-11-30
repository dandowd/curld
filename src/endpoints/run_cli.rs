use crate::endpoints::{
    endpoint_settings::{EndpointSettings, SavedEndpoint},
    utils::construct_curl_cmd,
};

pub fn run(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
    id: &Option<String>,
) -> String {
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
