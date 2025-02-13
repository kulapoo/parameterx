use std::{
    collections::BTreeMap, sync::Arc
};
use crate::{
    error::ParameterError,
    value::ParameterValue, Result,
};

#[derive(Debug, Default)]
pub struct Parameters {
    map: BTreeMap<String, Arc<dyn ParameterValue>>,
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        Self {
            map: self.map.iter()
                .map(|(k, v)| (k.clone(), v.clone_arc()))
                .collect()
        }
    }
}

impl Parameters {
    /// Create a new empty `Parameters` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// let params = Parameters::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a key-value pair into the `Parameters`.
    ///
    /// # Arguments
    ///
    /// * `key` - A key that can be converted into a `String`.
    /// * `value` - A value that implements the `ParameterValue` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// use parameterx::ParameterValue;
    ///
    /// #[derive(Debug, Clone)]
    /// struct MyValue;
    ///
    /// impl ToString for MyValue {
    ///     fn to_string(&self) -> String {
    ///         "my_value".to_string()
    ///     }
    /// }
    /// let mut params = Parameters::new();
    /// params.insert("key", MyValue);
    /// ```
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ParameterValue + 'static,
    {
        self.map.insert(key.into(), Arc::new(value));
    }

    /// Get a reference to a value of type `T` associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the value if found, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// use parameterx::ParameterValue;
    ///
   /// #[derive(Debug, Clone)]
    /// struct MyValue;
    ///
    /// impl ToString for MyValue {
    ///     fn to_string(&self) -> String {
    ///         "my_value".to_string()
    ///     }
    /// }
    ///
    /// let mut params = Parameters::new();
    /// params.insert("key", MyValue);
    /// let value: Option<&MyValue> = params.get("key");
    /// ```
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.map.get(key)
            .and_then(|value| value.as_any().downcast_ref::<T>())
    }

    /// Get a reference to a value of type `T` associated with the given key, or return an error if not found.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key.
    ///
    /// # Returns
    ///
    /// A `Result` containing a reference to the value if found, or a `ParameterError` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// use parameterx::ParameterValue;
    ///
   /// #[derive(Debug, Clone)]
    /// struct MyValue;
    ///
    /// impl ToString for MyValue {
    ///     fn to_string(&self) -> String {
    ///         "my_value".to_string()
    ///     }
    /// }
    ///
    /// let mut params = Parameters::new();
    /// params.insert("key", MyValue);
    /// let value: Result<&MyValue, _> = params.get_required("key");
    /// ```
    pub fn get_required<T: 'static>(&self, key: &str) -> Result<&T> {
        self.get(key).ok_or_else(|| ParameterError::KeyNotFound(key.to_string()))
    }

    /// Get the string representation of the value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key.
    ///
    /// # Returns
    ///
    /// An `Option` containing the string representation of the value if found, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// use parameterx::ParameterValue;
    ///
   /// #[derive(Debug, Clone)]
    /// struct MyValue;
    ///
    /// impl ToString for MyValue {
    ///     fn to_string(&self) -> String {
    ///         "my_value".to_string()
    ///     }
    /// }
    ///
    /// let mut params = Parameters::new();
    /// params.insert("key", MyValue);
    /// let value: Option<String> = params.get_string("key");
    /// ```
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.map.get(key).map(|value| value.to_string())
    }

    /// Check if the `Parameters` contains the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key.
    ///
    /// # Returns
    ///
    /// `true` if the key is present, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let mut params = Parameters::new();
    /// let exists: bool = params.contains_key("key");
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Try to get a value of type `T` associated with the given key, converting from a `String` if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key.
    ///
    /// # Returns
    ///
    /// A `Result` containing the value if found and successfully converted, or a `ParameterError` if not found or conversion failed.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let mut params = Parameters::new();
    /// params.insert("key", "123".to_string());
    /// let value: Result<String, _> = params.try_get("key");
    /// ```
    pub fn try_get<T>(&self, key: &str) -> Result<T>
    where
        T: TryFrom<String>,
        T::Error: std::error::Error + Send + Sync + 'static,
    {
        self.get_string(key)
            .ok_or_else(|| ParameterError::KeyNotFound(key.to_string()))?
            .try_into()
            .map_err(|e| ParameterError::ConversionFailed(Box::new(e)))
    }

    /// Insert a key-value pair into the `Parameters` and return the modified `Parameters`.
    ///
    /// # Arguments
    ///
    /// * `key` - A key that can be converted into a `String`.
    /// * `value` - A value that implements the `ParameterValue` trait.
    ///
    /// # Returns
    ///
    /// The modified `Parameters`.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    /// use parameterx::ParameterValue;
    ///
    /// #[derive(Debug, Clone)]
    /// struct MyValue;
    /// impl ToString for MyValue {
    ///     fn to_string(&self) -> String {
    ///         "MyValue".into()
    ///     }
    /// }
    ///
    /// let params = Parameters::new().with("key", MyValue);
    /// ```
    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: ParameterValue + 'static,
    {
        self.insert(key, value);
        self
    }

    /// Merge another `Parameters` instance into this one.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `Parameters` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let mut params1 = Parameters::new();
    /// let mut params2 = Parameters::new();
    /// params1.merge(params2);
    /// ```
    pub fn merge(&mut self, other: Parameters) {
        self.map.extend(other.map);
    }

    /// Get an iterator over the keys in the `Parameters`.
    ///
    /// # Returns
    ///
    /// An iterator over the keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let params = Parameters::new();
    /// for key in params.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    /// Get an iterator over the key-value pairs in the `Parameters`.
    ///
    /// # Returns
    ///
    /// An iterator over the key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let params = Parameters::new();
    /// for (key, value) in params.iter() {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<dyn ParameterValue>)> {
        self.map.iter()
    }

    /// Convert the `Parameters` to a JSON value.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON value if successful, or a `ParameterError` if conversion failed.
    ///
    /// # Examples
    ///
    /// ```
    /// use parameterx::Parameters;
    ///
    /// let params = Parameters::new();
    /// let json = params.to_json().unwrap();
    /// ```
    pub fn to_json(&self) -> Result<serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (key, value) in self.iter() {
            map.insert(key.clone(), value.to_json()?);
        }
        Ok(serde_json::Value::Object(map))
    }
}

// From implementations
impl From<String> for Parameters {
    fn from(text: String) -> Self {
        Parameters::new().with("text", text)
    }
}

impl From<&str> for Parameters {
    fn from(text: &str) -> Self {
        Parameters::new().with("text", text.to_string())
    }
}

impl FromIterator<(String, String)> for Parameters {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let mut params = Parameters::new();
        for (key, value) in iter {
            params.insert(key, value);
        }
        params
    }
}