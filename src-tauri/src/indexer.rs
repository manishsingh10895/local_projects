use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::Write,
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
    pub fn load() -> Result<Index, LpError> {
        let lp_home = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut home_path = PathBuf::from(lp_home);
        home_path.push("index.json");

        let exists = home_path.try_exists().unwrap_or(false);

        if !exists {
            return Err(LpError::Error(String::from("Not Found")));
        } else {
            let canonical = home_path.canonicalize().unwrap();

            match std::fs::File::open(&home_path) {
                Ok(file) => match serde_json::from_reader(file) {
                    Ok(json) => {
                        println!("Index Loaded From File {home_path:?}");
                        return Ok(json);
                    }
                    Err(err) => {
                        eprintln!("ERROR while parsing json");
                        eprintln!("{err}");
                        return Err(LpError::SerdeError(err));
                    }
                },
                Err(err) => {
                    eprintln!("ERROR: while opening file {canonical:?}");
                    return Err(LpError::IoError(err));
                }
            }
        }
    }

    pub fn load_or_default() -> Index {
        let lp_home = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut home_path = PathBuf::from(lp_home);
        home_path.push("index.json");

        let exists = home_path.try_exists().unwrap_or(false);

        if exists {
            let canonical = home_path.canonicalize().unwrap();
            match std::fs::File::open(&home_path) {
                Ok(file) => match serde_json::from_reader(file) {
                    Ok(json) => {
                        println!("Index Loaded From File {home_path:?}");
                        return json;
                    }
                    Err(err) => {
                        eprintln!("ERROR while parsing json");
                        eprintln!("{err}");
                        return Index::default();
                    }
                },
                Err(err) => {
                    eprintln!("ERROR: while opening file {canonical:?}");
                    eprintln!("{err}");
                    return Index::default();
                }
            }
        } else {
            eprintln!("ERROR: index.json file not found at {home_path:?}");
            return Index::default();
        }
    }

    /// Saves the index in a json file
    /// and updates `last_indexed` time
    pub fn save(&mut self) -> Result<(), LpError> {
        let lp_home = std::env::var("LP_CONFIG_PATH").unwrap_or(String::from("./"));
        let mut home_path = PathBuf::from(lp_home);
        home_path.push("index.json");

        let file: Result<File, std::io::Error> = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&home_path);

        self.last_indexed = SystemTime::now();

        match file {
            Ok(mut file) => {
                serde_json::to_writer(&file, &self)?;
                let canonical = home_path.canonicalize().unwrap();
                println!("Index saved to {canonical:?}");
                let _ = file.flush();
            }
            Err(err) => {
                println!("ERROR: {err}");
                return Err(LpError::IoError(err));
            }
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

#[cfg(test)]
mod indexer_tests {
    #[test]
    fn it_should_load_index() {
        let index = super::Index::load();

        assert!(index.is_ok());

        println!("Index Len -> {}", index.unwrap().projects().len());
    }
}
