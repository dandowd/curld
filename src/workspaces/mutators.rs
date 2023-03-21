use std::collections::HashMap;

use crate::variables::{parse, Extractor, Inserter};

use super::settings::Workspace;

static OPENING: &str = "$w{";
static CLOSING: &str = "}";

pub struct WorkspaceMutator {
    value_map: HashMap<String, String>,
}

impl WorkspaceMutator {
    pub fn new(workspace: &Workspace) -> WorkspaceMutator {
        WorkspaceMutator {
            value_map: workspace.variables.clone(),
        }
    }
}

impl Extractor for WorkspaceMutator {
    fn extract(&self, template: &str) -> Vec<String> {
        parse::extract_variable_names(template, OPENING, CLOSING)
    }
}

impl Inserter for WorkspaceMutator {
    fn insert(&self, template: &str, _value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(template, &self.value_map, OPENING, CLOSING)
    }
}
