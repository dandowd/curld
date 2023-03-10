use std::collections::{HashMap, HashSet};

pub fn extract_variable_names(templated: &str, opening: &str, closing: &str) -> Vec<String> {
    let opening_len = opening.len();
    // Use a HashSet to ensure there are no duplicates
    let mut names: HashSet<String> = HashSet::new();
    let mut alt_variabled = templated.to_owned();

    while let Some(start_index) = alt_variabled.find(opening) {
        let end_offset = match alt_variabled[start_index..].find(closing) {
            Some(index) => index,
            None => {
                panic!("Parsing error in variable: found opening brace but no closing")
            }
        };

        let end_index = start_index + end_offset;
        match alt_variabled[start_index + opening_len..end_index].find(opening) {
            Some(index) => panic!(
                "Parsing error in variable: found open bracket at {index}, expecting closing bracket",
                index = index
            ),
            None => 0,
        };

        let variable_name = String::from(&alt_variabled[start_index + opening_len..end_index]);
        names.insert(variable_name);

        alt_variabled = String::from(&alt_variabled[end_index + 1..]);
    }

    Vec::from_iter(names)
}

pub fn insert_variable_values(
    templated_str: &str,
    value_map: &HashMap<String, String>,
    opening: &str,
    closing: &str,
) -> String {
    let mut cloned_variabled_str = templated_str.to_owned();
    for (key, value) in value_map {
        let replace_key = format!("{0}{1}{2}", opening, key, closing);
        cloned_variabled_str = cloned_variabled_str.replace(&replace_key, value);
    }

    cloned_variabled_str
}

#[cfg(test)]
mod tests {
    use super::*;

    static VAR_OPEN: &str = "${";
    static VAR_CLOSE: &str = "}";

    static W_OPEN: &str = "$w{";
    static W_CLOSE: &str = "}";

    #[test]
    fn insert_workspace_variables() {
        let test_str = "-X ${METHOD} https://$w{base_url}/$w{version}/${endpoint}";
        let mut workspace_variables = HashMap::new();
        workspace_variables.insert("base_url".to_string(), "test.com".to_string());
        workspace_variables.insert("version".to_string(), "v1".to_string());

        let workspace_inserted =
            insert_variable_values(&test_str, &workspace_variables, W_OPEN, W_CLOSE);

        assert_eq!(
            workspace_inserted,
            "-X ${METHOD} https://test.com/v1/${endpoint}"
        );
    }

    #[test]
    fn extract_variable_names_should_parse() {
        let test_str = "-X ${method} https://${base_url}/v1/${endpoint}";

        let mut names = extract_variable_names(&test_str, VAR_OPEN, VAR_CLOSE);
        // Order the vector because the HashSet order is non-deterministic
        names.sort();
        assert_eq!(names.get(0).unwrap(), "base_url");
        assert_eq!(names.get(1).unwrap(), "endpoint");
        assert_eq!(names.get(2).unwrap(), "method");
    }

    #[test]
    #[should_panic]
    fn extract_variable_names_should_error_on_bad_parse() {
        let test_str = "https://${base_url/v1/${endpoint}";

        extract_variable_names(&test_str, VAR_OPEN, VAR_CLOSE);
    }

    #[test]
    fn extract_variable_names_should_parse_json() {
        let test_str = r#"'{ 'one': { 'sub': 'something' } ,'two': ${one} }'"#;

        let names = extract_variable_names(test_str, VAR_OPEN, VAR_CLOSE);

        assert_eq!(names.get(0).unwrap(), "one")
    }

    #[test]
    fn insert_variable_values_succeeds() {
        let test_str = "https://${base_url}/v1/${resource}/${resouceId}";
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("base_url".to_string(), "something.com".to_string());
        value_map.insert("resource".to_string(), "user".to_string());
        value_map.insert("resouceId".to_string(), "uuid".to_string());

        let replaced_str = insert_variable_values(&test_str, &value_map, VAR_OPEN, VAR_CLOSE);
        assert_eq!(replaced_str, "https://something.com/v1/user/uuid");
    }

    #[test]
    fn insert_variable_values_json() {
        let test_str = r#"{ "one": "${one_value}", "two": ${two_value} }"#;
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("one_value".to_string(), "first_value".to_string());
        value_map.insert("two_value".to_string(), "2".to_string());

        let replaced_str = insert_variable_values(&test_str, &value_map, VAR_OPEN, VAR_CLOSE);
        assert_eq!(replaced_str, r#"{ "one": "first_value", "two": 2 }"#);
    }
}
