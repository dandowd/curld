use std::collections::{HashMap, HashSet};

pub fn extract_template_names(templated: &str) -> Vec<String> {
    // Use a HashSet to ensure there are no duplicates
    let mut names: HashSet<String> = HashSet::new();
    let mut alt_templated = templated.to_owned();

    while let Some(start_index) = alt_templated.find("${") {
        let end_offset = match alt_templated[start_index..].find('}') {
            Some(index) => index,
            None => {
                panic!("Parsing error in template: found opening brace but no closing")
            }
        };

        let end_index = start_index + end_offset;
        match alt_templated[start_index + 2..end_index].find("${") {
            Some(index) => panic!(
                "Parsing error in template: found open bracket at {index}, expecting closing bracket",
                index = index
            ),
            None => 0,
        };

        let template_name = String::from(&alt_templated[start_index + 2..end_index]);
        names.insert(template_name);

        alt_templated = String::from(&alt_templated[end_index + 1..]);
    }

    Vec::from_iter(names)
}

pub fn insert_template_values(templated_str: &str, value_map: &HashMap<String, String>) -> String {
    let mut cloned_templated_str = templated_str.to_owned();
    for (key, value) in value_map {
        let replace_key = format!("${{{0}}}", key);
        cloned_templated_str = cloned_templated_str.replace(&replace_key, value);
    }

    cloned_templated_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_template_names_should_parse() {
        let test_str = "-X ${method} https://${base_url}/v1/${endpoint}";

        let mut names = extract_template_names(&test_str);
        // Order the vector because the HashSet order is non-deterministic
        names.sort();
        assert_eq!(names.get(0).unwrap(), "base_url");
        assert_eq!(names.get(1).unwrap(), "endpoint");
        assert_eq!(names.get(2).unwrap(), "method");
    }

    #[test]
    #[should_panic]
    fn extract_template_names_should_error_on_bad_parse() {
        let test_str = "https://${base_url/v1/${endpoint}";

        extract_template_names(&test_str);
    }

    #[test]
    fn extract_template_names_should_parse_json() {
        let test_str = r#"'{ 'one': { 'sub': 'something' } ,'two': ${one} }'"#;

        let names = extract_template_names(test_str);

        assert_eq!(names.get(0).unwrap(), "one")
    }

    #[test]
    fn insert_template_values_succeeds() {
        let test_str = "https://${base_url}/v1/${resource}/${resouceId}";
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("base_url".to_string(), "something.com".to_string());
        value_map.insert("resource".to_string(), "user".to_string());
        value_map.insert("resouceId".to_string(), "uuid".to_string());

        let replaced_str = insert_template_values(&test_str, &value_map);
        assert_eq!(replaced_str, "https://something.com/v1/user/uuid");
    }

    #[test]
    fn insert_template_values_json() {
        let test_str = r#"{ "one": "${one_value}", "two": ${two_value} }"#;
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("one_value".to_string(), "first_value".to_string());
        value_map.insert("two_value".to_string(), "2".to_string());

        let replaced_str = insert_template_values(&test_str, &value_map);
        assert_eq!(replaced_str, r#"{ "one": "first_value", "two": 2 }"#);
    }
}
