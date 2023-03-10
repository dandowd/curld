use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::mutators::VariableMutators;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct VariablesBuilder {
    #[serde(default)]
    pub keys: Vec<String>,

    #[serde(default)]
    pub value_map: HashMap<String, String>,

    #[serde(default)]
    pub original_args: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
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

    pub fn extract_keys(&mut self, curl_cmd: &Vec<String>) -> &mut Self {
        let variable_names: Vec<String> = curl_cmd
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

#[cfg(test)]
mod tests {}
