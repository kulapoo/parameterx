use crate::{Parameters, value::ParameterValue};

#[derive(Default)]
pub struct ParametersBuilder {
    params: Parameters,
}

impl ParametersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: ParameterValue + 'static,
    {
        self.params.insert(key, value);
        self
    }

    pub fn merge(mut self, other: Parameters) -> Self {
        self.params.merge(other);
        self
    }

    pub fn build(self) -> Parameters {
        self.params
    }
}