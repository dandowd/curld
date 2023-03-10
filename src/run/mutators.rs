use std::collections::HashMap;

use crate::variables::parse;

pub fn extract_variable_names(templated: &String) -> Vec<String> {
    parse::extract_variable_names(templated, "${", "}")
}

pub fn insert_values(templates: &String, value_map: &HashMap<String, String>) -> String {
    parse::insert_variable_values(templates, value_map, "${", "}")
}
