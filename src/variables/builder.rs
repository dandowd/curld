use crate::common::CurldCommand;

use super::{Extractor, Inserter};

#[derive(Clone)]
pub struct VariablesBuilder<'a> {
    inserters: Vec<&'a dyn Inserter>,
    extractors: Vec<&'a dyn Extractor>,
}

impl<'a> VariablesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inserters: Vec::new(),
            extractors: Vec::new(),
        }
    }

    pub fn extract(&mut self, user_args: &Vec<String>) -> Vec<String> {
        user_args
            .iter()
            .flat_map(|input| {
                self.extractors
                    .iter()
                    .flat_map(|extractor| extractor.extract(input))
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    pub fn insert(&self, curld: &CurldCommand) -> Vec<String> {
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
    fn extract_keys_should_get_var_name() {
        let mock_extractor = setup_extractor(2);
        let mut builder = VariablesBuilder::new();
        builder.add_extractor(&mock_extractor);

        let user_args = vec!["{{key}}".to_string(), "{{value}}".to_string()];
        let keys = builder.extract(&user_args);

        assert_eq!(keys, vec!["key".to_string(), "value".to_string(),]);
    }

    #[test]
    fn cmd_should_insert_values() {
        let mock_inserter = setup_inserter(2);

        let mut builder = VariablesBuilder::new();
        builder.add_inserter(&mock_inserter);

        let curld = CurldCommand {
            user_args: vec!["{{key}}".to_string(), "{{value}}".to_string()],
            value_map: HashMap::new(),
        };

        let cmd = builder.insert(&curld);

        assert_eq!(cmd, vec!["replaced".to_string(), "replaced".to_string()]);
    }
}
