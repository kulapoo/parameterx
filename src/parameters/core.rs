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
    /// Create a new empty Parameters instance
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ParameterValue + 'static,
    {
        self.map.insert(key.into(), Arc::new(value));
    }

    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.map.get(key)
            .and_then(|value| value.as_any().downcast_ref::<T>())
    }

    pub fn get_required<T: 'static>(&self, key: &str) -> Result<&T> {
        self.get(key).ok_or_else(|| ParameterError::KeyNotFound(key.to_string()))
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.map.get(key).map(|value| value.to_string())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn contains_type<T: 'static>(&self, key: &str) -> bool {
        self.map.get(key)
            .map(|value| value.as_any().is::<T>())
            .unwrap_or(false)
    }

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

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: ParameterValue + 'static,
    {
        self.insert(key, value);
        self
    }

    pub fn merge(&mut self, other: Parameters) {
        self.map.extend(other.map);
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<dyn ParameterValue>)> {
        self.map.iter()
    }

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