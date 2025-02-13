
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Parameter not found: {0}")]
    KeyNotFound(String),

    #[error("Type conversion failed: {0}")]
    ConversionFailed(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Type mismatch: expected {expected}, found {actual}")]
    TypeMismatch {
        expected: &'static str,
        actual: &'static str,
    },
}
