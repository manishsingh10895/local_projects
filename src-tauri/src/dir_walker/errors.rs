use std::fmt::Display;

use crate::errors::lp_error::LpError;

#[derive(Debug)]
pub enum DirError {
    Error(String),
}

impl std::error::Error for DirError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }

    fn description(&self) -> &str {
        match self {
            DirError::Error(_) => "Some error has occured",
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}

impl Display for DirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirError::Error(err) => write!(f, "{err}"),
        }
    }
}

impl From<DirError> for std::io::Error {
    fn from(value: DirError) -> Self {
        let kind = match value {
            DirError::Error(_) => std::io::ErrorKind::Other,
        };

        std::io::Error::new(kind, value)
    }
}

impl From<serde_json::Error> for LpError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<serde_yaml::Error> for LpError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::YamlError(value)
    }
}
