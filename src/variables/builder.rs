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
        if self.keys.is_empty() {
            return curld.user_args.to_owned();
        }

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
