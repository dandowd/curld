use std::collections::HashMap;

use crate::variables::{parse, Extractor, Inserter};

pub struct RunMutators {}

impl RunMutators {
    pub fn get_inserters() -> Vec<Inserter> {
        vec![Self::inserter]
    }

    pub fn get_extractors() -> Vec<Extractor> {
        vec![Self::extractor]
    }

    fn inserter(templated: &String, value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(templated, value_map, "${", "}")
    }

    fn extractor(templated: &String) -> Vec<String> {
        parse::extract_variable_names(templated, "${", "}")
    }
}
