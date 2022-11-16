use std::collections::{HashMap, HashSet};

pub struct Templates {
    pub endpoint: Option<Vec<String>>,
    pub data: Option<Vec<String>>,
    pub base_url: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
}

pub fn get_template_keys(
    endpoint: &String,
    data: &String,
    base_url: &String,
    headers: &String,
) -> Templates {
    let endpoint_templates = extract_template_names(endpoint).ok();
    let data_templates = extract_template_names(data).ok();
    let base_url_templates = extract_template_names(base_url).ok();
    let header_templates = extract_template_names(headers).ok();

    Templates {
        endpoint: endpoint_templates,
        data: data_templates,
        base_url: base_url_templates,
        headers: header_templates,
    }
}

pub fn construct_curl_endpoint(
    endpoint: &String,
    method: &String,
    data: &Option<String>,
    base_url: &Option<String>,
    headers: &Option<Vec<String>>,
) -> String {
    let url = match base_url {
        Some(base_url) => format!(
            " {base_url}{endpoint}",
            base_url = base_url,
            endpoint = endpoint
        ),
        None => format!(" {}", endpoint.to_string()),
    };

    let header_str = match headers {
        Some(headers) => {
            let mut header_str = String::new();
            for header in headers {
                header_str.push_str(&format!(" --header '{}'", header))
            }
            header_str
        }
        None => "".to_string(),
    };

    let data_str = match data {
        Some(data) => format!(" --data '{}'", data.to_string()),
        None => "".to_string()
    }; 

    format!(
        "--request {method}{headers}{data}{url}",
        method = method,
        headers = header_str,
        data = data_str,
        url = url
    )
}

pub fn extract_template_names(templated: &String) -> Result<Vec<String>, String> {
    // Use a HashSet to ensure there are no duplicates
    let mut names: HashSet<String> = HashSet::new();
    let mut alt_templated = templated.clone();
    loop {
        let start_index = match alt_templated.find("${") {
            Some(index) => index,
            None => break,
        };

        let end_index = match alt_templated.find("}") {
            Some(index) => index,
            None => {
                return Err(
                    "Parsing error in template: found opening brace but no closing".to_string(),
                )
            }
        };

        match alt_templated[start_index + 2..end_index].find("${") {
            Some(index) => return Err(
                format!(
                        "Parsing error in template: found open bracket at {index}, expecting closing bracket", 
                         index = index
                    )
                ),
            None => 0
        };

        let template_name = String::from(&alt_templated[start_index + 2..end_index]);
        names.insert(template_name);

        alt_templated = String::from(&alt_templated[end_index + 1..]);
    }

    Ok(Vec::from_iter(names))
}

pub fn insert_template_values(templated_str: &String, value_map: HashMap<String, String>) -> String {
    let mut cloned_templated_str = templated_str.clone();
    for (key, value) in value_map {
        let replace_key = format!("${{{0}}}", key);
        cloned_templated_str = cloned_templated_str.replace(&replace_key, &value);
    }

    cloned_templated_str
}
