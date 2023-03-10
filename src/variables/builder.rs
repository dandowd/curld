use std::collections::HashMap;

use crate::common::CurldCommand;

use super::mutators::VariableMutators;

#[derive(Clone)]
pub struct VariablesBuilder {
    pub keys: Vec<String>,

    pub value_map: HashMap<String, String>,

    pub original_args: Vec<String>,

    variable_mutators: VariableMutators,
}

impl VariablesBuilder {
    pub fn new(mutators: &VariableMutators) -> Self {
        Self {
            keys: Vec::new(),
            original_args: Vec::new(),
            value_map: HashMap::new(),
            variable_mutators: mutators.to_owned(),
        }
    }

    pub fn fill(&mut self, raw_command: &CurldCommand) -> &mut Self {
        self.keys = raw_command.keys.to_owned();
        self.value_map = raw_command.value_map.to_owned();
        self.original_args = raw_command.original_args.to_owned();

        self
    }

    pub fn set_original_args(&mut self, curl_cmd: &Vec<String>) -> &mut Self {
        self.original_args = curl_cmd.to_owned();
        self
    }

    pub fn build_curld_cmd(&self) -> CurldCommand {
        CurldCommand {
            keys: self.keys.to_owned(),
            value_map: self.value_map.to_owned(),
            original_args: self.original_args.to_owned(),
        }
    }

    pub fn extract_keys(&mut self) -> &mut Self {
        let variable_names: Vec<String> = self
            .original_args
            .iter()
            .flat_map(|input| {
                self.variable_mutators
                    .extractors
                    .iter()
                    .flat_map(|func| func(input))
                    .collect::<Vec<String>>()
            })
            .collect();

        self.keys.extend(variable_names);
        self
    }
    pub fn set_value_map(&mut self, value_map: &HashMap<String, String>) -> &mut Self {
        self.value_map = value_map.to_owned();
        self
    }

    pub fn cmd(&self) -> Vec<String> {
        if self.keys.is_empty() {
            return self.original_args.to_owned();
        }

        self.original_args
            .iter()
            .map(|input| {
                self.variable_mutators
                    .inserters
                    .iter()
                    .fold(input.to_owned(), |acc, func| func(&acc, &self.value_map))
            })
            .collect()
    }

    pub fn build_string(&self) -> String {
        self.variable_mutators
            .inserters
            .iter()
            .fold(self.original_args.join(" "), |acc, func| {
                func(&acc, &self.value_map)
            })
    }
}
