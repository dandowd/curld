use std::collections::HashMap;

pub mod builder;
pub mod mutators;
pub mod parse;

pub type Extractor = fn(input: &String) -> Vec<String>;
pub type Inserter = fn(input: &String, value_map: &HashMap<String, String>) -> String;

#[cfg(test)]
mod tests {
    use super::builder::VariablesBuilder;
    use super::mutators::VariableMutators;
    use super::*;

    fn setup_mutators() -> VariableMutators {
        let mock_extractor: Extractor = |x: &String| parse::extract_variable_names(x, "{", "}");
        let extractors = vec![mock_extractor];

        let mock_inserter: Inserter = |x: &String, value_map: &HashMap<String, String>| {
            parse::insert_variable_values(x, value_map, "{", "}")
        };

        let inserters = vec![mock_inserter];

        let mut mutators: VariableMutators = VariableMutators::new();
        mutators.register_extractors(extractors);
        mutators.register_inserters(inserters);

        mutators
    }

    #[test]
    fn should_extract_keys() {
        let curl_cmd: Vec<String> = vec!["-X", "{METHOD}", "https://{base_url}/{endpoint}"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let mutators = setup_mutators();

        let mut builder = VariablesBuilder::new(&mutators);
        builder.set_original_args(&curl_cmd).extract_keys();

        assert!(builder.keys.contains(&"METHOD".to_string()));
        assert!(builder.keys.contains(&"base_url".to_string()));
        assert!(builder.keys.contains(&"endpoint".to_string()));
    }

    #[test]
    fn should_insert_values() {
        let curl_cmd: Vec<String> = vec!["-X", "{METHOD}", "https://{base_url}/{endpoint}"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let mutators = setup_mutators();
        let mut builder = VariablesBuilder::new(&mutators);
        let value_map = HashMap::from([
            ("METHOD".to_string(), "POST".to_string()),
            ("base_url".to_string(), "test.com".to_string()),
        ]);

        let cmd = builder
            .set_original_args(&curl_cmd)
            .set_value_map(&value_map)
            .build_string();

        assert_eq!(cmd, "-X POST https://test.com/{endpoint}");
    }
}
