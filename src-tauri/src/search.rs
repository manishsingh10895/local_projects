use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::get_config_dir;
use crate::search_model::Model;
use serde_json;

pub fn create_search_index() {
    let mut config = get_config_dir();

    config.push("index.json");

    let mut model = Model::default();

    if let Ok(file) = std::fs::File::open(config) {
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();

        let projects = json.get("projects").unwrap();

        for (key, value) in projects.as_object().unwrap() {
            let path = key.as_str();

            let name = value.get("name").unwrap().as_str().unwrap();

            let project_type = match value.get("project_type") {
                Some(project_type) => {
                    if !project_type.is_null() {
                        let value = project_type.as_str().unwrap();
                        String::from(value)
                    } else {
                        String::from("")
                    }
                }
                None => String::from(""),
            };

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
                        let _secs = modified.get("secs_since_epoch").unwrap().as_u64().unwrap();
                        let _nanos = modified.get("nanos_since_epoch");

                        SystemTime::now()
                    } else {
                        SystemTime::now()
                    }
                }
                None => SystemTime::now(),
            };

            let mut content: String = String::new();

            // Repeat name. project_type to increase weightage

            for _ in 1..10 {
                content.push_str(&format!("{name} "));
            }

            content.push('\n');

            for _ in 0..3 {
                content.push_str(&format!("{project_type} "));
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

        let _res = save_model_as_json(&model, &index_path);
    }
}

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
