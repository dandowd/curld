use std::collections::HashMap;

use crate::variables::{parse, Extractor, Inserter};

pub struct RunMutators {}

impl Inserter for RunMutators {
    fn insert(&self, templated: &str, value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(templated, value_map, "${", "}")
    }
}

impl Extractor for RunMutators {
    fn extract(&self, templated: &str) -> Vec<String> {
        parse::extract_variable_names(templated, "${", "}")
    }
}
