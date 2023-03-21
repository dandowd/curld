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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_extract_keys() {
        let workspace = Workspace {
            name: "test".to_owned(),
            variables: HashMap::new(),
            commands: Vec::new(),
        };

        let mutator = WorkspaceMutator::new(&workspace);
        let keys = mutator.extract("$w{key} $w{key2}");

        assert_eq!(keys, vec!["key".to_owned(), "key2".to_owned()]);
    }

    #[test]
    fn it_should_not_extract_other_templates() {
        let workspace = Workspace {
            name: "test".to_owned(),
            variables: HashMap::new(),
            commands: Vec::new(),
        };

        let mutator = WorkspaceMutator::new(&workspace);
        let keys = mutator.extract("$w{key} ${key2}");

        assert_eq!(keys, vec!["key".to_owned()]);
    }
}
