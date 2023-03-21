#[cfg(test)]
use mockall::*;

use std::collections::HashMap;

pub mod builder;
pub mod parse;

#[cfg_attr(test, automock)]
pub trait Inserter {
    fn insert(&self, template: &str, value_map: &HashMap<String, String>) -> String;
}

#[cfg_attr(test, automock)]
pub trait Extractor {
    fn extract(&self, template: &str) -> Vec<String>;
}

#[cfg(test)]
mod tests {}
