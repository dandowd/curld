use crate::variables::mutators::VariableMutators;

pub struct Config {}

impl Config {
    pub fn get_mutators() -> VariableMutators {
        VariableMutators::new()
    }
}
