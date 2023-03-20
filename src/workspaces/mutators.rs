use std::collections::HashMap;

use crate::variables::{parse, Extractor, Inserter};

static OPENING: &str = "$w{";
static CLOSING: &str = "}";

pub struct WorkspaceMutators {}

impl WorkspaceMutators {
    pub fn get_extractors() -> Vec<Extractor> {
        vec![Self::extractor]
    }

    pub fn get_inserters() -> Vec<Inserter> {
        vec![Self::inserter]
    }

    fn extractor(templated: &String) -> Vec<String> {
        parse::extract_variable_names(templated, OPENING, CLOSING)
    }

    fn inserter(templated: &String, value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(templated, value_map, OPENING, CLOSING)
    }
}
