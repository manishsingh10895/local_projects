use std::{
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::config::get_config_dir;

pub mod config;
pub mod dir_walker;
use search_model::Model;
pub mod errors;
pub mod file_handler;
pub mod indexer;
pub mod lexer;
pub mod search_model;

fn main() {
    let mut config = get_config_dir();

    config.push("index.json");

    let mut model = Model::default();

    if let Ok(file) = std::fs::File::open(config) {
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();

        let projects = json.get("projects").unwrap();

        for (key, value) in projects.as_object().unwrap() {
            println!("key -> {key}");

            let path = key.as_str();

            let name = value.get("name").unwrap().as_str().unwrap();

            let description = match value.get("description") {
                Some(des) => {
                    if !des.is_null() {
                        let value = des.as_str().unwrap();
                        String::from(value)
                    } else {
                        String::from("")
                    }
                }
                None => String::from(""),
            };

            let doc_contents = match value.get("documentation_file") {
                Some(file) => {
                    if file.is_null() {
                        String::from("")
                    } else {
                        let path = PathBuf::from(file.as_str().unwrap());

                        match std::fs::read_to_string(path) {
                            Ok(content) => content,
                            Err(_) => String::from(""),
                        }
                    }
                }
                None => String::from(""),
            };

            let last_modified: SystemTime = match value.get("last_modified") {
                Some(modified) => {
                    if !modified.is_null() {
                        let _ = modified.get("secs_since_epoch").unwrap().as_u64().unwrap();
                        let _ = modified.get("nanos_since_epoch");

                        SystemTime::now()
                    } else {
                        SystemTime::now()
                    }
                }
                None => SystemTime::now(),
            };

            let mut content: String = String::new();

            for _ in 1..10 {
                content.push_str(&format!("{name} "));
            }

            content.push('\n');

            for _ in 1..5 {
                content.push_str(&format!("{description}"));
            }

            content.push('\n');

            content.push_str(&doc_contents);

            let content: Vec<char> = content.chars().into_iter().collect();

            model.add_document(PathBuf::from(path), last_modified, &content.as_slice());
        }

        let mut index_path = get_config_dir();

        index_path.push("search-index.json");

        let _ = save_model_as_json(&model, &index_path);
    }
}

/// Save `TermFreqIndex` to a json file
fn save_model_as_json(model: &Model, index_path: &Path) -> Result<(), ()> {
    println!("Saving {index_path:?}...");

    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create index file {index_path:?}: {err}");
    })?;

    serde_json::to_writer(BufWriter::new(index_file), &model).map_err(|err| {
        eprintln!("ERROR: could not serialze index into file {index_path:?}: {err}");
    })?;

    Ok(())
}

#[allow(dead_code)]
/// Loads search model from index file
fn load_model() -> Result<Model, ()> {
    let mut config = get_config_dir();
    config.push("search-index.json");
    if config.exists() {
        let index_file = fs::File::open(&config).map_err(|err| {
            eprintln!("ERROR: could not open {config:?}: {err}");
        })?;

        let model_data: Model = serde_json::from_reader(index_file).map_err(|err| {
            eprintln!("ERROR: could not parse search index file {config:?}: {err}")
        })?;

        Ok(model_data)
    } else {
        eprintln!("Cannot File search index at {config:?}");
        return Err(());
    }
}

#[cfg(test)]
mod index_tests {
    use crate::{config::get_config_dir, load_model, main};

    #[test]
    fn it_should_index_files() {
        main();

        let mut index_path = get_config_dir();
        index_path.push("search-index.json");

        let model = load_model();

        assert_eq!(model.is_ok(), true);

        let model = model.unwrap();

        let query: &[char] = &['r', 'u', 's', 't'];

        let res = model.search_query(query);

        assert!(res.is_ok());

        let res = res.unwrap();

        println!("{res:?}");

        assert!(index_path.exists());
    }
}
