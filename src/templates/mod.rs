use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod parse;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct TemplateBuilder {
    #[serde(default)]
    pub keys: Vec<String>,

    #[serde(default)]
    pub values: HashMap<String, String>,

    #[serde(default)]
    pub original: String,
}

impl TemplateBuilder {
    pub fn new(curl_cmd: &str) -> Self {
        let template_names = parse::extract_template_names(curl_cmd);

        Self {
            keys: template_names,
            original: curl_cmd.to_owned(),
            values: HashMap::new(),
        }
    }

    pub fn insert_values(&mut self, value_map: &HashMap<String, String>) {
        self.values = value_map.to_owned();
    }

    pub fn cmd(&self) -> String {
        if self.keys.is_empty() {
            return self.original.to_owned();
        }

        parse::insert_template_values(&self.original, &self.values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_templates_from_cmd() {
        let curl_cmd = r#"'--request ${method} { 'one': ${one}, 'two': 'no_two', 'three': '${three}' }' https://${base_url}.com/${env}/resource"#;

        let templates = TemplateBuilder::new(curl_cmd);

        assert!(templates.keys.contains(&String::from("method")));
        assert!(templates.keys.contains(&String::from("one")));
        assert!(templates.keys.contains(&String::from("three")));
        assert!(templates.keys.contains(&String::from("base_url")));
        assert!(templates.keys.contains(&String::from("env")));
    }

    #[test]
    fn should_return_cmd_with_inserted_values() {
        let curl_cmd = r#"--request ${method} { 'one': ${one}, 'two': 'no_two', 'three': '${three}' }' https://${base_url}.com/${env}/resource"#;
        let mut template = TemplateBuilder::new(curl_cmd);
        let value_map = HashMap::from([
            ("method".to_string(), "GET".to_string()),
            ("one".to_string(), "one_value".to_string()),
            ("three".to_string(), "three_value".to_string()),
            ("base_url".to_string(), "test.com".to_string()),
            ("env".to_string(), "test_env".to_string()),
        ]);

        template.insert_values(&value_map);

        assert_eq!(
            template.cmd(),
            r#"--request GET { 'one': one_value, 'two': 'no_two', 'three': 'three_value' }' https://test.com.com/test_env/resource"#
        );
    }
}
