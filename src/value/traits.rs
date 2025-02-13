use std::{
    any::Any,
    fmt::Debug,
    sync::Arc,
};
use serde_json;
use crate::{error::ParameterError, Result};

pub trait ParameterValue: Send + Sync + Debug {
    fn to_string(&self) -> String;
    fn type_name(&self) -> &'static str;
    fn clone_arc(&self) -> Arc<dyn ParameterValue>;
    fn as_any(&self) -> &dyn Any;

    fn to_json(&self) -> Result<serde_json::Value> {
        Err(ParameterError::ConversionFailed(
            "JSON serialization not implemented for this type".into()
        ))
    }
}

impl<T> ParameterValue for T
where
    T: Send + Sync + Debug + Clone + ToString + Any + 'static
{
    fn to_string(&self) -> String {
        self.to_string()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn clone_arc(&self) -> Arc<dyn ParameterValue> {
        Arc::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}