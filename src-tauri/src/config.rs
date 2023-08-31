// App config

use std::{io::BufWriter, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::{config_error::ConfigError, lp_error::LpError};

/// Config for the LocalProjects App
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// List of directory, that would be searched
    /// for different projects
    pub project_dirs: Vec<PathBuf>,
}

impl Config {
    /// Loads config from `lp.config.json`
    /// sets `project_dirs` to empty array
    /// if `config file` `lp.config.json` is not found
    pub fn load() -> Self {
        let mut dirs = Vec::new();

        let path = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut path = PathBuf::from(path);
        path.push("lp.config.json");

        let file = std::fs::File::open(path).expect("Config File should be opened");
        let config: Config = serde_json::from_reader(file).expect("Deserialized Config from file");

        dirs = config.project_dirs;

        Config { project_dirs: dirs }
    }

    pub fn new() -> Self {
        Config {
            project_dirs: Vec::new(),
        }
    }

    /// Add a directory to the config
    pub fn add_dir(&mut self, path: String) -> Result<(), LpError> {
        if let Some(_) = self
            .project_dirs
            .iter()
            .find(|d| path == d.to_string_lossy())
        {
            return Err(LpError::ConfigError(ConfigError::PathExists));
        }

        self.project_dirs.push(path.into());

        Ok(())
    }

    /// Removes a directory from project config
    pub fn remove_dir(&mut self, path: String) -> Result<(), LpError> {
        self.project_dirs.retain(|d| path != d.to_string_lossy());

        Ok(())
    }

    pub fn save(&self) -> Result<(), LpError> {
        if let Ok(path) = std::env::var("LP_CONFIG_PATH") {
            let file = std::fs::File::open(path).expect("Config File should be opened");
            serde_json::to_writer(BufWriter::new(file), self)
                .expect("Serialize to save Config file");
        };
        Ok(())
    }
}

#[cfg(test)]
mod config_tests {
    use super::Config;
    #[test]
    fn it_should_read_config_file_create_config_struct() {
        dotenv::dotenv().ok();

        let path = std::env::var("LP_CONFIG_PATH");

        let current_dir = std::env::current_dir().unwrap();

        println!("Current dir {current_dir:?}");

        println!("{path:?}");

        Config::load();
    }

    #[test]
    fn it_should_add_directory_to_config() {
        dotenv::dotenv().ok();

        let mut config = Config::new();

        let mut dir = std::env::home_dir().unwrap();

        dir.push("Documents");

        let mut doc_dir = std::env::home_dir().unwrap();
        doc_dir.push("Documents");

        let result = config.add_dir(dir.into_os_string().into_string().unwrap());

        assert!(result.is_ok());
        assert_eq!(config.project_dirs.len(), 1);
        assert!(config.project_dirs.first().is_some());
        assert_eq!(config.project_dirs.first().unwrap(), &doc_dir);
    }

    #[test]
    fn it_should_remove_directory_from_config() {
        dotenv::dotenv().ok();

        let mut config = Config::new();

        let mut dir = std::env::home_dir().unwrap();

        dir.push("Documents");

        let _ = config.add_dir(dir.clone().into_os_string().into_string().unwrap());

        assert_eq!(config.project_dirs.len(), 1);

        let _ = config.remove_dir(dir.into_os_string().into_string().unwrap());

        assert_eq!(config.project_dirs.len(), 0);
    }
}
