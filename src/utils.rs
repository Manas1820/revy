use glob;
use sha1::{Digest, Sha1};
use std::{fs, path::PathBuf};

pub const REPO_FOLDER_NAME: &str = ".revy";

/// Check if a directory exists at the specified path.
///
/// # Arguments
///
/// * `path` - A reference to a String containing the path to check.
///
/// # Returns
///
/// Returns true if the directory exists, false otherwise.
pub fn check_if_directory_exists(path: &String) -> bool {
    fs::metadata(path).is_ok()
}

/// Fetch the path for setting up a repository. If a repository name is provided, a directory with the
/// repository name is created.
///
/// # Arguments
///
/// * `repository_name` - An optional string slice containing the name of the repository.
///
/// # Returns
///
/// Returns a String containing the path for repository setup.
pub fn fetch_path_for_repository(repository_name: Option<&str>) -> String {
    let current_directory_path = std::env::current_dir().unwrap();
    let mut curent_working_directory = current_directory_path.to_str().unwrap().to_string();

    if let Some(repo_name) = repository_name {
        curent_working_directory.push_str(&format!("/{}", repo_name));
        fs::create_dir(&curent_working_directory).unwrap();
    }

    format!("{}/{}", curent_working_directory, REPO_FOLDER_NAME)
}

/// Initialize a repository at the specified path.
///
/// # Arguments
///
/// * `current_repo_initiation_path` - A string slice containing the path for repository initialization.
///
/// # Panics
///
/// Panics if directory or file creation fails.
pub fn initialize_repository(current_repo_initiation_path: &str) {
    fs::create_dir(&current_repo_initiation_path).unwrap();
    fs::create_dir(format!("{}/objects", &current_repo_initiation_path)).unwrap();
    fs::create_dir(format!("{}/refs", &current_repo_initiation_path)).unwrap();
    fs::write(
        format!("{}/HEAD", &current_repo_initiation_path),
        "ref: refs/head/main\n",
    )
    .unwrap();
}

/// Generate a SHA1 hash for the provided object data.
///
/// # Arguments
/// * `data` - A reference to a String containing the data to hash.
///
/// # Returns
/// Returns a String containing the SHA1 hash.
pub fn generate_sha1(data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Fetch Excluded Paths
pub fn fetch_excluded_paths() -> Vec<String> {
    // TODO: Implement this function to fetch excluded paths from the .revyignore file
    // and check if a local .revyignore file exists in the current directory and ignore those too

    //Objectives:

    /*
        - Check if a .revyignore file exists in the current directory
        - Read the contents of the .revyignore file
        - Parse the contents of the .revyignore file
        - Return the list of excluded paths
    */

    let mut excluded_paths: Vec<String> = Vec::new();

    let global_revy_ignore_path = PathBuf::from("./.revyignore");

    if check_if_directory_exists(&global_revy_ignore_path.to_str().unwrap().to_string()) {
        let global_revy_ignore_contents = fs::read_to_string(global_revy_ignore_path).unwrap();
        let global_revy_ignore_lines = global_revy_ignore_contents.lines();

        for line in global_revy_ignore_lines {
            if line.trim().starts_with("#") {
                continue;
            }

            let mut line = line.to_string();
            line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            line = line.split("#").collect::<Vec<&str>>()[0].to_string();

            excluded_paths.push(line.to_string());
        }
    }
    excluded_paths
}

/// Check if a path should be ignored.

pub fn should_ignore(path: &std::path::Path, patterns: &[String]) -> bool {
    // TODO: Implement a better way to check if a path should be ignored.

    // println!("Path : {:?} , Pattern: {:?}", path, patterns);
    let path_str = path.to_str().unwrap();
    patterns.iter().any(|pattern| {
        glob::Pattern::new(pattern).unwrap().matches_path(path)
            || path_str.contains(pattern.trim_end_matches('/'))
    })
}
