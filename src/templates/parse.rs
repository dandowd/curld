use std::collections::{HashSet, HashMap};

pub fn extract_template_names(templated: &str) -> Result<Vec<String>, String> {
    // Use a HashSet to ensure there are no duplicates
    let mut names: HashSet<String> = HashSet::new();
    let mut alt_templated = templated.to_owned();

    while let Some(start_index) = alt_templated.find("${") {
        let end_index = match alt_templated.find('}') {
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
        let templated = insert_template_values(str, value_map);
        values.push(templated);
    }

    values
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
        let test_str = "https://${base_url}/v1/${endpoint}";

        let mut names = extract_template_names(&test_str).unwrap();
        // Order the vector because the HashSet order is non-deterministic
        names.sort();
        assert_eq!(names.get(0).unwrap(), "base_url");
        assert_eq!(names.get(1).unwrap(), "endpoint")
    }

    #[test]
    fn extract_template_names_should_error_on_bad_parse() {
        let test_str = "https://${base_url/v1/${endpoint}";

        match extract_template_names(&test_str) {
            Ok(_) => panic!("extract template names succeeded when it should fail"),
            Err(err) => assert!(!err.is_empty()),
        };
    }

    #[test]
    fn insert_template_values_succeeds() {
        let test_str = "https://${base_url}/v1/${resource}/${resouceId}";
        let mut value_map: HashMap<String, String> = HashMap::new();
        value_map.insert("base_url".to_string(), "something.com".to_string());
        value_map.insert("resource".to_string(), "user".to_string());
        value_map.insert("resouceId".to_string(), "uuid".to_string());

        let replaced_str = insert_template_values(&test_str, &value_map);
        assert_eq!(replaced_str, "https://something.com/v1/user/uuid")
    }
}
