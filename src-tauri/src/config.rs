// App config

use dirs::{self, config_dir, home_dir};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::errors::{config_error::ConfigError, lp_error::LpError};

/// Returns path for LP_CONFIG directory
pub fn get_config_dir() -> PathBuf {
    let mut base = match config_dir() {
        Some(path) => path,
        None => home_dir().unwrap(),
    };

    base.push(PathBuf::from(".lp_config/"));

    if !base.exists() {
        let _ = fs::create_dir_all(&base);
    }

    base
}

/// Config for the LocalProjects App
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// List of directory, that would be searched
    /// for different projects
    pub project_dirs: Vec<PathBuf>,
}

impl Config {
    pub fn clone(&self) -> Config {
        Config {
            project_dirs: self.project_dirs.clone(),
        }
    }

    /// Loads config from `lp.config.json`
    /// sets `project_dirs` to empty array
    /// if `config file` `lp.config.json` is not found
    pub fn load() -> Self {
        let mut path = get_config_dir();

        path.push(PathBuf::from("lp.config.json"));

        println!("Loading config from {path:?}");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();

        let config: Config = match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("[ERROR] {e:?}");

                return Config::new();
            }
        };

        let dirs = config.project_dirs;

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

        println!("[config.add_dir] path added");
        Ok(())
    }

    /// Removes a directory from project config
    pub fn remove_dir(&mut self, path: String) -> Result<(), LpError> {
        self.project_dirs.retain(|d| path != d.to_string_lossy());

        Ok(())
    }

    pub fn save(&self) -> Result<(), LpError> {
        let mut path = get_config_dir();

        path.push(PathBuf::from("lp.config.json"));

        let config = self.clone();

        let config_json = serde_json::to_string_pretty(&config)?;

        std::fs::write(&path, &config_json)?;

        println!("config saved at {path:?}", path = path.clone());
        Ok(())
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::get_config_dir;

    use super::Config;
    #[test]
    fn it_should_read_config_file_create_config_struct() {
        dotenv::dotenv().ok();

        let path = get_config_dir();

        let current_dir = std::env::current_dir().unwrap();

        println!("Current dir {current_dir:?}");

        println!("{path:?}");

        Config::load();
    }

    #[test]
    fn it_should_add_directory_to_config() {
        dotenv::dotenv().ok();

        let mut config = Config::load();

        #[allow(deprecated)]
        let mut dir = std::env::home_dir().unwrap();

        dir.push("Documents");

        #[allow(deprecated)]
        let mut doc_dir = std::env::home_dir().unwrap();
        doc_dir.push("Documents");

        let result = config.add_dir(dir.into_os_string().into_string().unwrap());

        let saved = config.save();

        assert!(result.is_ok());
        assert!(saved.is_ok());
        assert_eq!(config.project_dirs.len(), 1);
        assert!(config.project_dirs.first().is_some());
        assert_eq!(config.project_dirs.first().unwrap(), &doc_dir);
    }

    #[test]
    fn it_should_remove_directory_from_config() {
        dotenv::dotenv().ok();

        let mut config = Config::new();

        #[allow(deprecated)]
        let mut dir = std::env::home_dir().unwrap();

        dir.push("Documents");

        let _ = config.add_dir(dir.clone().into_os_string().into_string().unwrap());

        assert_eq!(config.project_dirs.len(), 1);

        let _ = config.remove_dir(dir.into_os_string().into_string().unwrap());

        assert_eq!(config.project_dirs.len(), 0);
    }
}
