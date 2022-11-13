use std::collections::HashSet;

pub struct Templates {
    endpoint: Option<Vec<String>>,
    method: Option<String>,
    data: Option<Vec<String>>,
    base_url: Option<Vec<String>>,
    headers: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct InputError {
    pub details: String,
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

fn extract_template_names(templated: &String) -> Result<Vec<String>, InputError> {
    // Use a HashSet to ensure there are no duplicates
    let mut names: HashSet<String> = HashSet::new();
    let mut alt_templated = templated.clone();
    loop {
        let start_index = match alt_templated.find("{") {
            Some(index) => index,
            None => break,
        };

        let end_index = match alt_templated.find("}") {
            Some(index) => index,
            None => {
                return Err(InputError {
                    details: String::from(
                        "Parsing error in template: found opening brace but no closing",
                    ),
                })
            }
        };

        match alt_templated[start_index + 1..end_index].find("{") {
            Some(index) => return Err(InputError {
                details: format!("Parsing error in template: found open bracket at {index}, expecting closing bracket", index = index)
            }),
            None => 0
        };

        let template_name = String::from(&alt_templated[start_index + 1..end_index]);
        names.insert(template_name);

        alt_templated = String::from(&alt_templated[end_index + 1..]);
    }

    Ok(Vec::from_iter(names))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_template_names_should_parse() {
        let test_str = String::from("https://{base_url}/v1/{endpoint}");

        let mut names = extract_template_names(&test_str).unwrap();
        // Order the vector because the HashSet order is non-deterministic
        names.sort();
        assert_eq!(names.get(0).unwrap(), "base_url");
        assert_eq!(names.get(1).unwrap(), "endpoint")
    }

    #[test]
    fn extract_template_names_should_error_on_bad_parse() {
        let test_str = String::from("https://{base_url/v1/{endpoint}");

        match extract_template_names(&test_str) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }
}
