use super::{Extractor, Inserter};

#[derive(Clone, Default)]
pub struct VariableMutators {
    pub extractors: Vec<Extractor>,
    pub inserters: Vec<Inserter>,
}

impl VariableMutators {
    pub fn new() -> Self {
        Self {
            extractors: Vec::new(),
            inserters: Vec::new(),
        }
    }
    pub fn register_extractors(&mut self, extractors: Vec<Extractor>) {
        self.extractors.extend(extractors);
    }

    pub fn register_inserters(&mut self, inserters: Vec<Inserter>) {
        self.inserters.extend(inserters);
    }
}
