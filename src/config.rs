use crate::{
    run::mutators::{extract_variable_names, insert_values},
    variables::{mutators::VariableMutators, Extractor, Inserter},
};

pub struct Config {}

impl Config {
    pub fn get_mutators() -> VariableMutators {
        let mut variable_mutator = VariableMutators::new();

        let run_extractor: Extractor = extract_variable_names;
        let run_inserter: Inserter = insert_values;

        let extractors = vec![run_extractor];
        let inserters = vec![run_inserter];

        variable_mutator.register_extractors(extractors);
        variable_mutator.register_inserters(inserters);

        variable_mutator
    }
}
