use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    path::PathBuf,
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::{errors::lp_error::LpError, file_handler::Project};

#[derive(Serialize, Deserialize)]
pub struct Index {
    projects: BTreeMap<PathBuf, Project>,
    last_indexed: SystemTime,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            projects: BTreeMap::new(),
            last_indexed: SystemTime::now(),
        }
    }
}

impl Index {
    pub fn load_or_default() -> Index {
        let lp_home = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut home_path = PathBuf::from(lp_home);
        home_path.push("index.json");

        let exists = home_path.try_exists().unwrap_or(false);

        if exists {
            if let Ok(file) = std::fs::File::open(home_path) {
                if let Ok(json) = serde_json::from_reader(file) {
                    return json;
                } else {
                    return Index::default();
                }
            } else {
                return Index::default();
            }
        } else {
            return Index::default();
        }
    }

    /// Saves the index in a json file
    /// and updates `last_indexed` time
    pub fn save(&mut self) -> Result<(), LpError> {
        let lp_home = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut home_path = PathBuf::from(lp_home);
        home_path.push("index.json");

        let file: Result<File, std::io::Error> =
            OpenOptions::new().create(true).write(true).open(home_path);

        self.last_indexed = SystemTime::now();

        if let Ok(file) = file {
            serde_json::to_writer(file, &self)?;
        } else {
            return Err(file.unwrap_err().into());
        }

        Ok(())
    }

    pub fn add_project(&mut self, path: &PathBuf, project: Project) {
        self.projects.remove(path);

        self.projects.insert(path.to_path_buf(), project);
    }

    /// Returns projects as `Vec<Project>`
    pub fn projects(&self) -> Vec<Project> {
        let projects: Vec<Project> = self.projects.iter().map(|(_, p)| p.clone()).collect();

        projects
    }

    /// Return `BTreeMap` of projects
    pub fn projects_map(&self) -> &BTreeMap<PathBuf, Project> {
        &self.projects
    }

    /// A project requires reindexing
    /// * If it is not already present in the index
    /// * And the directory is modified after being indexed
    pub fn should_reindex(&self, path: &PathBuf, last_modified: SystemTime) -> bool {
        if let Some(project) = self.projects.get(path) {
            return project.last_modified < last_modified;
        }

        true
    }

    pub fn last_indexed(&self) -> SystemTime {
        self.last_indexed
    }
}

pub struct Indexer {}
