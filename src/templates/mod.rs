use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod parse;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct TemplateBuilder {
    #[serde(default)]
    pub keys: Vec<String>,

    #[serde(default)]
    pub value_map: HashMap<String, String>,

    #[serde(default)]
    pub original_args: Vec<String>,
}

impl TemplateBuilder {
    pub fn new(curl_cmd: Vec<String>) -> Self {
        let template_names = curl_cmd
            .iter()
            .flat_map(|input| parse::extract_template_names(input))
            .collect();

        Self {
            keys: template_names,
            original_args: curl_cmd,
            value_map: HashMap::new(),
        }
    }

    pub fn insert_values(&mut self, value_map: &HashMap<String, String>) {
        self.value_map = value_map.to_owned();
    }

    pub fn cmd(&self) -> Vec<String> {
        if self.keys.is_empty() {
            return self.original_args.to_owned();
        }

        self.original_args
            .iter()
            .map(|field| parse::insert_template_values(field, &self.value_map))
            .collect()
    }

    pub fn build_string(&self) -> String {
        parse::insert_template_values(&self.original_args.join(" "), &self.value_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_string() {
        let curl_cmd: Vec<String> = [
            "-X",
            "POST",
            "-d",
            r#"{ "one": "${thing}" }"#,
            "https://${base_url}",
        ]
        .map(String::from)
        .to_vec();

        let value_map = HashMap::from([
            ("thing".to_string(), "one_value".to_string()),
            ("base_url".to_string(), "test.com".to_string()),
        ]);

        let mut templates = TemplateBuilder::new(curl_cmd);
        templates.insert_values(&value_map);

        assert_eq!(
            templates.build_string(),
            r#"-X POST -d { "one": "one_value" } https://test.com"#
        )
    }

    #[test]
    fn should_extract_templates_from_cmd() {
        let curl_cmd = vec![
            "--request".to_string(),
            "${method}".to_string(),
            "-d".to_string(),
            "'{ 'one': ${one}, 'two': 'no_two', 'three': '${three}' }'".to_string(),
            "https://${base_url}.com/${env}/resource".to_string(),
        ];

        let templates = TemplateBuilder::new(curl_cmd);

        assert!(templates.keys.contains(&String::from("method")));
        assert!(templates.keys.contains(&String::from("one")));
        assert!(templates.keys.contains(&String::from("three")));
        assert!(templates.keys.contains(&String::from("base_url")));
        assert!(templates.keys.contains(&String::from("env")));
    }

    #[test]
    fn should_return_cmd_with_inserted_values() {
        let curl_cmd = vec![
            "--request".to_string(),
            "${method}".to_string(),
            "-d".to_string(),
            r#"{ "one": ${one}, "two": "no_two", "three": "${three}" }"#.to_string(),
            "https://${base_url}.com/${env}/resource".to_string(),
        ];
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
            vec![
                "--request",
                "GET",
                "-d",
                r#"{ "one": one_value, "two": "no_two", "three": "three_value" }"#,
                "https://test.com.com/test_env/resource"
            ]
        );
    }
}
