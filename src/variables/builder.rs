use crate::common::CurldCommand;

use super::{Extractor, Inserter};

#[derive(Clone)]
pub struct VariablesBuilder<'a> {
    pub keys: Vec<String>,

    inserters: Vec<&'a dyn Inserter>,
    extractors: Vec<&'a dyn Extractor>,
}

impl<'a> VariablesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            inserters: Vec::new(),
            extractors: Vec::new(),
        }
    }

    pub fn extract_keys(&mut self, user_args: &Vec<String>) -> Vec<String> {
        let keys = user_args
            .iter()
            .flat_map(|input| {
                self.extractors
                    .iter()
                    .flat_map(|extractor| extractor.extract(input))
                    .collect::<Vec<String>>()
            })
            .collect();

        self.keys = keys;
        self.keys.clone()
    }

    pub fn cmd(&self, curld: &CurldCommand) -> Vec<String> {
        curld
            .user_args
            .iter()
            .map(|input| {
                self.inserters
                    .iter()
                    .fold(input.to_owned(), |acc, inserter| {
                        inserter.insert(&acc, &curld.value_map)
                    })
            })
            .collect()
    }

    pub fn to_string(&self, curld: &CurldCommand) -> String {
        self.inserters
            .iter()
            .fold(curld.user_args.join(" "), |acc, inserter| {
                inserter.insert(&acc, &curld.value_map)
            })
    }

    pub fn add_inserter(&mut self, inserter: &'a dyn Inserter) {
        self.inserters.push(inserter);
    }

    pub fn add_extractor(&mut self, extractor: &'a dyn Extractor) {
        self.extractors.push(extractor);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::variables::{MockExtractor, MockInserter};

    fn setup_extractor(call_times: usize) -> MockExtractor {
        let mut extractor = MockExtractor::new();
        extractor
            .expect_extract()
            .times(call_times)
            .returning(|input| vec![input.replace("{{", "").replace("}}", "")]);
        extractor
    }

    fn setup_inserter(call_times: usize) -> MockInserter {
        let mut inserter = MockInserter::new();
        inserter
            .expect_insert()
            .times(call_times)
            .returning(|_, _| "replaced".to_string());
        inserter
    }

    #[test]
    fn test_extract_keys() {
        let mock_extractor = setup_extractor(2);
        let mut builder = VariablesBuilder::new();
        builder.add_extractor(&mock_extractor);

        let user_args = vec!["{{key}}".to_string(), "{{value}}".to_string()];
        let keys = builder.extract_keys(&user_args);

        assert_eq!(keys, vec!["key".to_string(), "value".to_string(),]);
    }

    #[test]
    fn cmd_should_insert_values_after_extract() {
        let mock_inserter = setup_inserter(2);
        let mock_extractor = setup_extractor(2);

        let mut builder = VariablesBuilder::new();
        builder.add_inserter(&mock_inserter);
        builder.add_extractor(&mock_extractor);

        let curld = CurldCommand {
            user_args: vec!["{{key}}".to_string(), "{{value}}".to_string()],
            value_map: HashMap::new(),
        };

        builder.extract_keys(&curld.user_args);

        let cmd = builder.cmd(&curld);

        assert_eq!(cmd, vec!["replaced".to_string(), "replaced".to_string()]);
    }
}
