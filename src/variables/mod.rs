use std::collections::HashMap;

pub mod builder;
pub mod mutators;
pub mod parse;

pub type Extractor = fn(input: &String) -> Vec<String>;
pub type Inserter = fn(input: &String, value_map: &HashMap<String, String>) -> String;
