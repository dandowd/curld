use std::collections::{HashMap, HashSet};

pub fn construct_curl_cmd(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
) -> String {
    let url = format!(
        " {base_url}{endpoint}",
        base_url = base_url,
        endpoint = endpoint
    );

    let header_str = {
        let mut header_str = String::new();
        for header in headers {
            header_str.push_str(&format!(" --header '{}'", header))
        }
        header_str
    };

    let data_str = if data != "" {
        format!(" --data '{}'", data.to_string())
    } else {
        "".to_string()
    };

    format!(
        "--request -X {method}{headers}{data}{url}",
        method = method,
        headers = header_str,
        data = data_str,
        url = url
    )
}

pub fn construct_curl_args(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    args.push("-X".to_string());
    args.push(method.to_string());

    let header_str = {
        let mut header_str = String::new();
        for header in headers {
            header_str.push_str(&format!(" --header '{}'", header))
        }
        header_str
    };

    if header_str.len() > 0 {
        args.push(header_str);
    }

    if data != "" {
        args.push("-d".to_string());
        args.push( data.to_string());
    };

    // Url should always be last
    let url = format!(
        "{base_url}{endpoint}",
        base_url = base_url,
        endpoint = endpoint
    );
    args.push(url);

    args
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

pub fn insert_template_values_vec(
    vec_str: &Vec<String>,
    value_map: &HashMap<String, String>,
) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for str in vec_str {
        let templated = insert_template_values(&str, value_map);
        values.push(templated);
    }

    values
}

pub fn insert_template_values(
    templated_str: &String,
    value_map: &HashMap<String, String>,
) -> String {
    let mut cloned_templated_str = templated_str.clone();
    for (key, value) in value_map {
        let replace_key = format!("${{{0}}}", key);
        cloned_templated_str = cloned_templated_str.replace(&replace_key, &value);
    }

    cloned_templated_str
}
