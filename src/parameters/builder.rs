use crate::{Parameters, value::ParameterValue};

#[derive(Default)]
pub struct ParametersBuilder {
    params: Parameters,
}

/// A builder for creating `Parameters` instances.
///
/// # Examples
///
/// ```
/// use parameterx::ParametersBuilder;
/// let params = ParametersBuilder::new()
///     .add("key1", "value1")
///     .add("key2", 42)
///     .build();
/// ```
///
/// # Methods
///
/// - `new`: Creates a new `ParametersBuilder` instance.
/// - `add`: Adds a key-value pair to the parameters. The key must implement `Into<String>` and the value must implement `ParameterValue`.
/// - `merge`: Merges another `Parameters` instance into the builder.
/// - `build`: Consumes the builder and returns the constructed `Parameters` instance.
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