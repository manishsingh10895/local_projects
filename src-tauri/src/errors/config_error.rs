use std::fmt::Display;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    PathExists,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => {
                write!(f, "CONFIG_ERROR: {err}")
            }
            Self::PathExists => {
                write!(f, "CONFIG_ERROR: Path already exists")
            }
        }
    }
}
