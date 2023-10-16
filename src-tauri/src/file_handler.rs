use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::{config::Config, dir_walker, errors::lp_error::LpError, indexer::Index};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cache {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Struct Describing a Dev Project
pub struct Project {
    /// Name of the project
    pub name: String,
    /// Location of the project
    pub path: String,
    /// Git repo url for the project
    pub git: Vec<String>,
    /// Description, may be null
    pub description: Option<String>,
    /// A hashmap containing, programming languages and their
    /// correspoding percentage share of the code
    pub language_map: HashMap<String, f32>,
    /// Type of project , Rust, Flutter, NextJs etc.
    pub project_type: ProjectType,

    /// Last Modified type for project
    pub last_modified: SystemTime,

    /// Documentantion file for the project
    /// Mostly Readme.md
    pub documentation_file: Option<String>,
}

impl Project {
    pub fn base(
        name: String,
        path: String,
        description: Option<String>,
        project_type: ProjectType,
    ) -> Self {
        Project {
            name,
            path,
            git: Vec::new(),
            description,
            language_map: HashMap::new(),
            project_type,
            last_modified: SystemTime::now(),
            documentation_file: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ProjectType {
    Rust,
    Python,
    Flutter,
    Ruby,
    NextJs,
    Svelte,
    React,
    ReactNative,
    Angular,
    Node,
    Vue,
}

/// Searches for projects under `project_dirs`
pub fn initiate_search(config: &Config) -> Result<(), LpError> {
    println!("Initating Search");
    println!("For Projects {:?}", config.project_dirs);

    let index_path = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));

    let mut index_path = PathBuf::from(index_path);

    index_path.push("index.json");

    let index: Arc<Mutex<Index>> = Arc::new(Mutex::new(Index::load_or_default()));

    dir_walker::walker::analyze_all_dirs(config, 4, 4, Arc::clone(&index));

    index.lock().unwrap().save()?;

    let index_len = index.lock().unwrap().projects().len();

    println!("Index Saved -> Len, {}", index_len);

    Ok(())
}

#[cfg(test)]
mod file_tests {
    // #[ignore]
    #[test]
    pub fn it_should_create_an_index() {
        std::env::set_var("LP_CONFIG_PATH", "./");

        let config = super::Config::load();
        let res = super::initiate_search(&config);

        println!("{res:?}");

        assert!(res.is_ok());
    }
}
