use std::collections::HashMap;

use crate::variables::{parse, Inserter};

use super::settings::Workspace;

static OPENING: &str = "w{";
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

impl Inserter for WorkspaceMutator {
    fn insert(&self, template: &str, _value_map: &HashMap<String, String>) -> String {
        parse::insert_variable_values(template, &self.value_map, OPENING, CLOSING)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserter_should_insert_values_for_workspace() {
        let workspace = Workspace {
            name: "test".to_string(),
            variables: vec![("key".to_string(), "value".to_string())]
                .into_iter()
                .collect(),
            commands: vec![],
        };
        let mutator = WorkspaceMutator::new(&workspace);

        let result = mutator.insert("test w{key}", &HashMap::new());

        assert_eq!("test value", result);
    }
}
