use std::collections::{HashMap, HashSet};

pub struct Templates {
    endpoint: Option<Vec<String>>,
    data: Option<Vec<String>>,
    base_url: Option<Vec<String>>,
    headers: Option<Vec<String>>,
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
    endpoint: String,
    method: String,
    data: Option<String>,
    base_url: Option<String>,
    headers: Option<Vec<String>>,
) -> String {
    let url = match base_url {
        Some(base_url) => format!(
            "{base_url}/{endpoint}",
            base_url = base_url,
            endpoint = endpoint
        ),
        None => endpoint,
    };

    let header_str = match headers {
        Some(headers) => {
            let mut header_str = String::new();
            for header in headers {
                header_str.push_str(&format!("--header {}", header))
            }
            header_str
        }
        None => "".to_string(),
    };

    let data_str = data.unwrap_or("".to_string());

    format!(
        "--request {method}{headers}{data} {url}",
        method = method,
        headers = header_str,
        data = data_str,
        url = url
    )
}

fn extract_template_names(templated: &String) -> Result<Vec<String>, String> {
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

fn insert_template_values(templated_str: &String, value_map: HashMap<String, String>) -> String {
    let mut cloned_templated_str = templated_str.clone();
    for (key, value) in value_map {
        let replace_key = format!("${{{0}}}", key);
        cloned_templated_str = cloned_templated_str.replace(&replace_key, &value);
    }

    cloned_templated_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_template_names_should_parse() {
        let test_str = String::from("https://${base_url}/v1/${endpoint}");

        let mut names = extract_template_names(&test_str).unwrap();
        // Order the vector because the HashSet order is non-deterministic
        names.sort();
        assert_eq!(names.get(0).unwrap(), "base_url");
        assert_eq!(names.get(1).unwrap(), "endpoint")
    }

    #[test]
    fn extract_template_names_should_error_on_bad_parse() {
        let test_str = String::from("https://${base_url/v1/${endpoint}");

        match extract_template_names(&test_str) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn insert_template_values_succeeds() {
        let test_str = String::from("https://${base_url}/v1/${resource}/${resouceId}");
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("base_url".to_string(), "something.com".to_string());
        value_map.insert("resource".to_string(), "user".to_string());
        value_map.insert("resouceId".to_string(), "uuid".to_string());

        let replaced_str = insert_template_values(&test_str, value_map);
        assert_eq!(replaced_str, "https://something.com/v1/user/uuid")
    }

    #[test]
    fn get_template_keys_returns_templates() {
        let base_url = String::from("https://${base_url}/${env}/v1");
        let endpoint = String::from("/${resouce}/${resourceId}");
        let headers =
            String::from("--header 'content-type: ${type} --header x-api-key: ${api-key}'");
        let data = String::from("{ \"resourceId\": \"${resourceId}\", \"number\": ${number} }");

        let templates = get_template_keys(&endpoint, &data, &base_url, &headers);
        assert!(templates.base_url.unwrap().iter().all(|item| [
            "base_url".to_string(),
            "env".to_string()
        ]
        .contains(item)));

        assert!(templates.endpoint.unwrap().iter().all(|item| [
            "resouce".to_string(),
            "resourceId".to_string()
        ]
        .contains(item)));

        assert!(templates.headers.unwrap().iter().all(|item| [
            "type".to_string(),
            "api-key".to_string()
        ]
        .contains(item)));

        assert!(templates.data.unwrap().iter().all(|item| [
            "resourceId".to_string(),
            "number".to_string()
        ]
        .contains(item)));
    }
}
