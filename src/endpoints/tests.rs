use std::collections::HashMap;

use crate::endpoints::utils::{extract_template_names, insert_template_values};

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

    let replaced_str = insert_template_values(&test_str, &value_map);
    assert_eq!(replaced_str, "https://something.com/v1/user/uuid")
}
