use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crossbeam_channel::Sender;

use crate::{
    config::Config,
    errors::lp_error::LpError,
    file_handler::{Project, ProjectType},
    indexer::Index,
};

struct Job(PathBuf, usize, Sender<Job>);

use toml::Table;

const EXCLUDE_DIRS: [&str; 6] = [".git", "node_modules", "target", ".vscode", "src", "venv"];

///  Entry point to analysis
pub fn analyze_all_dirs(
    config: &Config,
    threads: usize,
    max_depth: usize,
    index: Arc<Mutex<Index>>,
) -> Vec<Project> {
    println!("Using {threads} for analyzing dirs");
    {
        let (job_sender, job_receiver) = crossbeam_channel::unbounded::<Job>();

        let (result_sender, result_receiver) = crossbeam_channel::unbounded::<Project>();
        let cloned_index = Arc::clone(&index);
        (0..threads)
            .map(|_| (job_receiver.clone(), result_sender.clone()))
            .for_each(|(jr, rs)| {
                let clone = Arc::clone(&cloned_index);
                std::thread::spawn(move || {
                    jr.into_iter().for_each(|job| {
                        scan_dir(
                            &job.0,
                            job.1,
                            Arc::clone(&clone),
                            max_depth,
                            job.2,
                            rs.clone(),
                        )
                    })
                });
            });

        config.project_dirs.iter().for_each(|p| {
            job_sender
                .send(Job(p.to_path_buf(), 0, job_sender.clone()))
                .unwrap();
        });

        result_receiver
    }
    .into_iter()
    .collect()
}

/// Get Relevant Details about a project if any
/// `files` : Array of file paths in `dir`
fn get_relevant_project(
    files: &Vec<PathBuf>,
    dir: &PathBuf,
    last_modified: SystemTime,
) -> Option<Project> {
    const PROJECT_IDENTIFIERS: [&str; 8] = [
        "Cargo.toml",
        "package.json",
        "pubspec.yaml",
        "main.go",
        "main.py",
        "next.config.js",
        "svelte.config.json",
        "angular.json",
    ];

    let len = files.len();

    let mut id_file: Option<&PathBuf> = None;

    // Look for doc file, right now only README.md or DOC.md
    //
    let mut doc_file = None;

    // Check if any file is a Project Identifier
    // and extract ID file
    for i in 0..len {
        if let Some(name) = files[i].file_name() {
            let name = name.to_str().unwrap();
            if PROJECT_IDENTIFIERS.contains(&name) {
                id_file = Some(&files[i]);
                break;
            }

            if name.to_lowercase() == "readme.md" || name.to_lowercase() == "doc.md" {
                println!("DOC FILE found for {dir:?}");
                doc_file = Some(files[i].to_str().unwrap().to_string());
            } else {
                println!("DOC FILE NOT FOUND for {dir:?}");
            }
        }
    }

    let mut project = None;

    if id_file.is_some() {
        if let Some(name) = id_file.unwrap().file_name() {
            let name = name.to_str().unwrap();
            let id_file = id_file.unwrap();

            match name {
                "Cargo.toml" => {
                    project = cargo_project(id_file, dir).ok();
                }
                "package.json" => {
                    project = node_project(id_file, files, dir).ok();
                }
                "pubspec.yaml" => {
                    project = flutter_project(id_file, dir).ok();
                }
                _ => {}
            }
        }
    }

    if project.is_some() {
        let remotes = get_git_remotes(dir);

        let language_map = get_project_language_map(dir);

        project = project.map(|mut p| {
            p.git = remotes;
            p.language_map = language_map;
            p.last_modified = last_modified;
            p.documentation_file = doc_file;
            p
        });
    }

    project
}

/// Get Details for a Rust(Cargo) Project
/// `cargo_path` -> File path for Cargo.toml
/// `project_dir` -> Project Path
fn cargo_project(cargo_path: &PathBuf, project_dir: &PathBuf) -> Result<Project, LpError> {
    let toml_content = fs::read_to_string(cargo_path)?;

    let mut project_name = project_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut project_desc = None;

    if let Ok(parsed) = toml::from_str::<Table>(&toml_content)
        .map_err(|err| eprintln!("ERROR: parsing toml content : {err}"))
    {
        if let Some(package) = parsed.get("package") {
            if let Some(name) = package.get("name") {
                project_name = name.as_str().unwrap().to_string();
            }

            project_desc = package.get("description").map(|d| d.to_string());
        }
    }

    Ok(Project::base(
        project_name,
        project_dir.to_str().unwrap().to_string(),
        project_desc,
        ProjectType::Rust,
    ))
}

// Get details for a flutter project
fn flutter_project(pubspec_file: &PathBuf, project_dir: &PathBuf) -> Result<Project, LpError> {
    // DirName as default project name
    // if name is not found in pacakge.json
    let mut project_name = project_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let yaml_content = std::fs::read_to_string(pubspec_file)?;

    let pubspec: serde_yaml::value::Value = serde_yaml::from_str(&yaml_content)?;

    if let Some(name) = pubspec.get("name") {
        project_name = name.as_str().unwrap().to_string();
    }

    let project_desc = pubspec
        .get("description")
        .map(|o| o.as_str().unwrap().to_string());

    Ok(Project::base(
        project_name,
        project_dir.to_str().unwrap().to_string(),
        project_desc,
        ProjectType::Flutter,
    ))
}

/// Get details for node/react/svelete or any Js project with package.json
fn node_project(
    package_json_file: &PathBuf,
    files: &Vec<PathBuf>,
    project_dir: &PathBuf,
) -> Result<Project, LpError> {
    // DirName as default project name
    // if name is not found in pacakge.json
    let mut project_name = project_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let json_content = std::fs::read_to_string(package_json_file)?;

    let package_json: serde_json::Value = serde_json::from_str(&json_content)?;

    if let Some(name) = package_json.get("name") {
        project_name = name.as_str().unwrap().to_string();
    }

    let project_desc = package_json.get("description").map(|o| o.to_string());

    // Defaults projet_type to Node
    let mut project_type = ProjectType::Node;

    let mut identifier_file: Option<&PathBuf> = None;
    const NODE_PROJECT_IDENTIFIERS: [&str; 8] = [
        "svelte.config.json",
        "svelte.config.js",
        "angular.json",
        "metro.config.js",
        "react-native.config.js",
        "next.config.js",
        "next.config.json",
        "vue.config.js",
    ];

    // search if any files are one of the NODE_PROJECT_IDENTIFIERS
    files.iter().enumerate().for_each(|(_, file)| {
        if let Some(name) = file.file_name() {
            let name = name.to_str().unwrap();

            if NODE_PROJECT_IDENTIFIERS.contains(&name) {
                identifier_file = Some(file);
            }
        }
    });

    if let Some(id_file) = identifier_file {
        match id_file.to_str().unwrap() {
            "svelte.config.json" | "svelte.config.js" => {
                project_type = ProjectType::Svelte;
            }
            "angular.json" => {
                project_type = ProjectType::Angular;
            }
            "next.config.js" | "next.config.json" => {
                project_type = ProjectType::NextJs;
            }
            "metro.config.js" | "react-native.config.js" => {
                project_type = ProjectType::ReactNative;
            }
            "vue.config.js" => project_type = ProjectType::Vue,
            _ => {}
        }
    }
    // If couldn't identify any other Project type
    // Check for dependencies
    if project_type == ProjectType::Node {
        if let Some(deps) = package_json.get("dependencies") {
            let deps = deps.as_object().unwrap();

            if deps.get("react").is_some() {
                project_type = ProjectType::React;
            } else if deps.get("@angular/core").is_some() {
                project_type = ProjectType::Angular;
            }
        }

        if let Some(dev_deps) = package_json.get("devDependencies") {
            let deps = dev_deps.as_object().unwrap();

            if deps.get("svelte").is_some() {
                project_type = ProjectType::Svelte;
            }
        }
    }

    Ok(Project::base(
        project_name,
        project_dir.to_str().unwrap().to_string(),
        project_desc,
        project_type,
    ))
}

/// Get a languge data map for a project
/// What langugages are used and in what percentage, like github
fn get_project_language_map(project_dir: &PathBuf) -> HashMap<String, f32> {
    let excluded = &["target", "node_modules", ".git"];

    let paths = &[project_dir];

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

    language_map
}

/// Get all `remote urls` for the project
/// if the project is a git repo
fn get_git_remotes(project_dir: &PathBuf) -> Vec<String> {
    let mut repo_remotes = Vec::new();

    if let Ok(repo) = git2::Repository::open(project_dir) {
        if let Ok(remotes) = repo.remotes() {
            remotes
                .iter()
                .filter(|r| r.is_some())
                .map(|remote| remote.unwrap())
                .map(|remote| repo.find_remote(remote))
                .filter(|remote| remote.is_ok())
                .map(|remote| remote.unwrap())
                .map(|remote| {
                    // following code required as to
                    // return an owned value `String`
                    if let Some(url) = remote.url() {
                        return Some(String::from(url));
                    } else {
                        return None;
                    }
                })
                .filter(|remote| remote.is_some())
                .map(|remote| remote.unwrap())
                .for_each(|r| repo_remotes.push(r));
        }
    }

    repo_remotes
}

fn scan_dir(
    path: &PathBuf,
    depth: usize,
    index: Arc<Mutex<Index>>,
    max_depth: usize,
    job_sender: Sender<Job>,
    result_sender: Sender<Project>,
) {
    println!("Scanning {path:?} as depth {depth}");

    let dot_file = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with("."))
        .unwrap_or(false);

    if dot_file {
        return;
    }

    let read_dir = match path.read_dir() {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("ERROR: reading directory failed for {path:?} {e:?}");
            return;
        }
    };

    // partition create two vector from above, if is_dir adds to
    // first vector and if not second vector
    // Consumes the iterator
    let (dirs, files): (Vec<_>, Vec<_>) = read_dir
        .filter_map(|it| it.ok().map(|it| it.path()))
        .partition(|it| it.is_dir());

    let mut last_modified = SystemTime::now();

    if let Ok(meta) = path.metadata() {
        if let Ok(modified) = meta.modified() {
            last_modified = modified;
        }
    }

    let mut found_project = false;

    // Check if the project is already indexed an up to date
    if index.lock().unwrap().should_reindex(path, last_modified) {
        if let Some(project) = get_relevant_project(&files, path, last_modified) {
            index.lock().unwrap().add_project(path, project.clone());
            result_sender.send(project).unwrap();
            found_project = true;
        } else {
            eprintln!("ERROR: cannot get relevant project for {path:?}");

            let file: Result<File, std::io::Error> = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("./errors.txt");

            if let Ok(mut file) = file {
                let _ = file.write_fmt(format_args!("{path:?}\n"));
            }
        }
    }

    println!("FOUND PROJECT {found_project} || Depth {depth}");

    if found_project == false && max_depth > depth {
        println!("\n\nIterating Dirs for {path:?}");
        for dir in dirs {
            let filename = dir.file_name().unwrap_or_default().to_str().unwrap();

            if EXCLUDE_DIRS.contains(&filename) {
                continue;
            }

            job_sender
                .send(Job(dir.to_path_buf(), depth + 1, job_sender.clone()))
                .unwrap();
        }
    } else {
        return;
    }
}

#[cfg(test)]
mod walker_tests {
    use super::{flutter_project, get_relevant_project};

    #[test]
    fn it_should_return_relevant_project_data() {
        let mut home = std::env::home_dir().unwrap();
        home.push("Documents/projects/rust/search_engine");

        let read_dir = match home.read_dir() {
            Ok(iter) => iter,
            Err(e) => {
                eprintln!("ERROR: reading directory failed for {home:?} {e:?}");
                return;
            }
        };

        let (_, files): (Vec<_>, Vec<_>) = read_dir
            .filter_map(|it| it.ok().map(|it| it.path()))
            .partition(|it| it.is_dir());

        // let _result = get_relevant_project(&files, &home);
    }

    #[test]
    fn it_should_get_details_for_a_flutter_project() {
        let mut flutter_project_path = std::env::home_dir().unwrap();

        flutter_project_path.push("Documents/template/bpmonitor");

        let mut pubspec_path = flutter_project_path.clone();

        pubspec_path.push("pubspec.yaml");

        let project = flutter_project(&pubspec_path, &flutter_project_path);

        assert!(project.is_ok());
    }
}
