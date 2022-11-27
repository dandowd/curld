use std::collections::HashMap;

use crate::endpoints::utils::{extract_template_names, get_template_keys};

use super::utils::insert_template_values;

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
    let headers = String::from("--header 'content-type: ${type} --header x-api-key: ${api-key}'");
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
