use std::{error::Error, fmt::Debug};

use serde::Serialize;
use tauri::InvokeError;

use crate::errors::config_error::ConfigError;

#[derive(Debug)]
pub enum LpError {
    ConfigError(ConfigError),
    Error(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    YamlError(serde_yaml::Error),
}

impl std::fmt::Display for LpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(err) => write!(f, "{err}"),
            Self::Error(err) => write!(f, "{err}"),
            Self::IoError(err) => {
                let source = err.source();
                let _ = write!(f, "{source:?}");
                write!(f, "{err}")
            }
            Self::SerdeError(err) => write!(f, "{err}"),
            Self::YamlError(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Serialize)]
struct ErrorWrapper {
    error: String,
}

impl From<InvokeError> for LpError {
    fn from(value: InvokeError) -> Self {
        let message = format!("{value:?}");
        LpError::Error(message.to_string())
    }
}

impl Serialize for LpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = "{self:?}";

        let wrapper = ErrorWrapper {
            error: message.to_string(),
        };

        wrapper.serialize(serializer)
    }
}

impl std::error::Error for LpError {}

impl From<std::io::Error> for LpError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
