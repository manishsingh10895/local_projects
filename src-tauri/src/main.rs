// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use config::Config;
use errors::lp_error::LpError;
use file_handler::Project;
pub mod config;
pub mod dir_walker;
pub mod errors;
pub mod file_handler;
pub mod indexer;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct AppState {
    config: Arc<Mutex<Config>>,
    index: Arc<Mutex<indexer::Index>>,
}

#[tauri::command]
fn config_add_dir(path: String, state: tauri::State<AppState>) -> Result<(), LpError> {
    let mut x = state.config.lock().unwrap();
    x.add_dir(path)
}

#[tauri::command]
fn get_config(state: tauri::State<AppState>) -> Config {
    let config = state.config.clone();

    let config = Arc::try_unwrap(config).unwrap().into_inner().unwrap();

    config
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

// Command to get projects

fn main() {
    dotenv::dotenv().ok();

    let config = Config::load();
    {
        std::thread::spawn(move || file_handler::initiate_search(&config));
    }
    tauri::Builder::default()
        .manage(AppState {
            config: Arc::new(Mutex::new(Config::load())),
            index: Arc::new(Mutex::new(indexer::Index::load_or_default())),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            config_add_dir,
            get_config,
            get_projects,
            reload_index
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod main_tests {
    use std::collections::HashMap;

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
