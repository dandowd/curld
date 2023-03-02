use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::parse;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct VariablesBuilder {
    #[serde(default)]
    pub keys: Vec<String>,

    #[serde(default)]
    pub value_map: HashMap<String, String>,

    #[serde(default)]
    pub original_args: Vec<String>,
}

impl VariablesBuilder {
    pub fn new(curl_cmd: Vec<String>) -> Self {
        let variable_names = curl_cmd
            .iter()
            .flat_map(|input| parse::extract_variable_names(input))
            .collect();

        Self {
            keys: variable_names,
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
            .map(|field| parse::insert_variable_values(field, &self.value_map))
            .collect()
    }

    pub fn build_string(&self) -> String {
        parse::insert_variable_values(&self.original_args.join(" "), &self.value_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_string_from_cmd_with_variables() {
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

        let mut variables = VariablesBuilder::new(curl_cmd);
        variables.insert_values(&value_map);

        assert_eq!(
            variables.build_string(),
            r#"-X POST -d { "one": "one_value" } https://test.com"#
        )
    }

    #[test]
    fn should_extract_variables_from_cmd() {
        let curl_cmd = vec![
            "-X".to_string(),
            "${method}".to_string(),
            "-d".to_string(),
            "'{ 'one': ${one}, 'two': 'no_two', 'three': '${three}' }'".to_string(),
            "https://${base_url}.com/${env}/resource".to_string(),
        ];

        let variables = VariablesBuilder::new(curl_cmd);

        assert!(variables.keys.contains(&String::from("method")));
        assert!(variables.keys.contains(&String::from("one")));
        assert!(variables.keys.contains(&String::from("three")));
        assert!(variables.keys.contains(&String::from("base_url")));
        assert!(variables.keys.contains(&String::from("env")));
    }

    #[test]
    fn should_return_cmd_with_inserted_values() {
        let curl_cmd = vec![
            "-X".to_string(),
            "${method}".to_string(),
            "-d".to_string(),
            r#"{ "one": ${one}, "two": "no_two", "three": "${three}" }"#.to_string(),
            "https://${base_url}.com/${env}/resource".to_string(),
        ];
        let mut variable = VariablesBuilder::new(curl_cmd);
        let value_map = HashMap::from([
            ("method".to_string(), "GET".to_string()),
            ("one".to_string(), "one_value".to_string()),
            ("three".to_string(), "three_value".to_string()),
            ("base_url".to_string(), "test.com".to_string()),
            ("env".to_string(), "test_env".to_string()),
        ]);

        variable.insert_values(&value_map);

        assert_eq!(
            variable.cmd(),
            vec![
                "-X",
                "GET",
                "-d",
                r#"{ "one": one_value, "two": "no_two", "three": "three_value" }"#,
                "https://test.com.com/test_env/resource"
            ]
        );
    }
}
