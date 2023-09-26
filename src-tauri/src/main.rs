// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use config::Config;
use errors::lp_error::LpError;
use file_handler::Project;
pub mod config;
pub mod dir_walker;
pub mod errors;
pub mod file_handler;
pub mod indexer;
pub mod lexer;
pub mod search;
pub mod search_model;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Maintaing tauri state
pub struct AppState {
    config: Arc<Mutex<Config>>,
    index: Arc<Mutex<indexer::Index>>,
    search_model: Arc<Mutex<search_model::Model>>,
    is_indexing: Arc<Mutex<bool>>,
}

#[tauri::command]
fn config_add_dir(path: String, state: tauri::State<AppState>) -> Result<(), LpError> {
    let mut conf = state.config.lock().unwrap();

    conf.add_dir(path)?;

    conf.save()
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Config {
    match state.config.lock() {
        Ok(_config) => {
            return _config.clone();
        }
        Err(err) => {
            eprintln!("ERROR: cannot get lock on config: {err}");
            panic!();
        }
    };
}

#[tauri::command]
fn get_file_contents(file: String) -> Result<String, LpError> {
    std::fs::read_to_string(file).map_err(|err| err.into())
}

#[tauri::command]
fn search_query(query: String, state: tauri::State<AppState>) -> Result<Vec<Project>, LpError> {
    let model = state.search_model.lock().unwrap();

    let query: Vec<char> = query.chars().into_iter().collect();

    let projects = model.search_query(&query.as_slice());

    println!("{projects:?}");

    Ok(Vec::new())
}

#[tauri::command]
fn get_projects(state: tauri::State<AppState>) -> Vec<Project> {
    if let Ok(index) = state.index.lock() {
        return index.projects();
    }

    return [].to_vec();
}

#[tauri::command]
fn reload_index(state: tauri::State<AppState>) {
    let new_index = indexer::Index::load_or_default();

    let mut index = state.index.lock().unwrap();

    *index = new_index;

    println!("Index Len {}", index.projects().len());

    println!("INDEX RELOADED");
}

#[tauri::command]
fn open_project(path: String) {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };

    let res = Command::new(shell).args(["code", &path]).spawn();

    if res.is_err() {
        panic!("Cannot open code");
    }
}

#[tauri::command]
fn re_index(state: tauri::State<AppState>) {
    let config = Arc::clone(&state.config);
    let mut indexing = state.is_indexing.lock().unwrap();
    *indexing = true;
    println!("[re_index] Indexing Start");

    let c = config.lock().unwrap().clone();
    let x = std::thread::spawn(move || file_handler::initiate_search(&c)).join();
    if let Ok(_) = x {
        *indexing = false;
    }

    println!("[re_index] Indexing Finished");
}

#[tauri::command]
fn is_indexing(state: tauri::State<AppState>) -> bool {
    *state.is_indexing.lock().unwrap()
}

// Command to get projects

fn main() {
    dotenv::dotenv().ok();

    let config = Config::load();
    {
        std::thread::spawn(move || file_handler::initiate_search(&config));
    }

    let model = match search_model::load_model() {
        Ok(model) => model,
        Err(_) => search_model::Model::default(),
    };

    tauri::Builder::default()
        .manage(AppState {
            config: Arc::new(Mutex::new(Config::load())),
            index: Arc::new(Mutex::new(indexer::Index::load_or_default())),
            search_model: Arc::new(Mutex::new(model)),
            is_indexing: Arc::new(Mutex::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            config_add_dir,
            get_config,
            get_projects,
            get_file_contents,
            reload_index,
            re_index,
            is_indexing,
            open_project,
            search_query,
        ])
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    println!("{matches:?}");
                }
                Err(err) => {
                    println!("{err:?}");
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod main_tests {
    use std::collections::HashMap;

    use crate::open_project;

    #[ignore]
    #[test]
    fn it_should_open_project() {
        let path = "/Users/s_mash/Documents/projects/rust/local_projects";

        let x = open_project(path.to_string());
    }

    #[ignore]
    #[test]
    fn it_should_get_language_details() {
        let excluded = &["target", "node_modules"];

        let paths = &["./"];

        let config = tokei::Config::default();

        let mut languages = tokei::Languages::new();

        languages.get_statistics(paths, excluded, &config);

        let keys: std::collections::btree_map::Keys<tokei::LanguageType, tokei::Language> =
            languages.keys();

        let mut language_map = HashMap::<String, f32>::new();

        let total_code_lines = languages.total().code as f32;
        for k in keys {
            let value = languages.get(k).unwrap();

            let lang_code_lines = value.code as f32;

            let percent = (lang_code_lines / total_code_lines) * 100.0;

            language_map.insert(k.to_string(), percent);
        }

        // NOTE: apply Largest Remainder method
        // to assure that the percentage add up to 100

        println!("LANG MAP");
        println!("{language_map:?}");
    }
}
