use std::collections::HashMap;

use crate::variables::{parse, Extractor, Inserter};

pub struct RunMutators {}

static OPENING: &str = "r{";
static CLOSING: &str = "}";

impl Inserter for RunMutators {
    fn insert(&self, templated: &str, value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(templated, value_map, OPENING, CLOSING)
    }
}

impl Extractor for RunMutators {
    fn extract(&self, templated: &str) -> Vec<String> {
        parse::extract_variable_names(templated, OPENING, CLOSING)
    }
}
