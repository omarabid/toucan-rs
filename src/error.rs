use std::fmt;
use thiserror::Error;

/// Result alias
pub type Result<T> = std::result::Result<T, ToucanError>;

/// Error type for this library.
#[derive(Error, Debug)]
pub struct ToucanError {
    pub msg: String,
    backtrace: Option<std::backtrace::Backtrace>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Implement the Display trait for our Error type.
impl fmt::Display for ToucanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<worker::Error> for ToucanError {
    fn from(err: worker::Error) -> Self {
        Self {
            msg: err.to_string(),
            backtrace: None,
            source: Some(Box::new(err)),
        }
    }
}

impl From<wasm_bindgen::JsValue> for ToucanError {
    fn from(err: wasm_bindgen::JsValue) -> Self {
        Self {
            msg: err
                .as_string()
                .unwrap_or_else(|| "JsValue string conversion failed".to_string()),
            backtrace: None,
            source: None,
        }
    }
}
