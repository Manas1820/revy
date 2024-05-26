use sha1::{Digest, Sha1};
use std::fs;

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

    return format!("{}/{}", curent_working_directory, REPO_FOLDER_NAME);
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
pub fn generate_sha1_for_object(data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
