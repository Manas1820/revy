use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commnds: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Intialize the revy repository
    Init,
}

const REPO_FOLDER_NAME: &str = ".revy";

/// Check if a directory exists at the specified path.
///
/// # Arguments
///
/// * `path` - A reference to a String containing the path to check.
///
/// # Returns
///
/// Returns true if the directory exists, false otherwise.
fn check_if_directory_exists(path: &String) -> bool {
    fs::metadata(path).is_ok()
}

/// Fetch the path for setting up a repository.
///
/// # Arguments
///
/// * `repository_name` - An optional string slice containing the name of the repository.
///
/// # Returns
///
/// Returns a String containing the path for repository setup.
fn fetch_path_for_repository_setup(repository_name: Option<&str>) -> String {
    let current_directory_path = std::env::current_dir().unwrap();
    let mut curent_working_directory = current_directory_path.to_str().unwrap().to_string();

    if let Some(repo_name) = repository_name {
        curent_working_directory.push_str(&format!("/{}", repo_name));
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
fn initialize_repository(current_repo_initiation_path: &str) {
    fs::create_dir(&current_repo_initiation_path).unwrap();
    fs::create_dir(format!("{}/objects", &current_repo_initiation_path)).unwrap();
    fs::create_dir(format!("{}/refs", &current_repo_initiation_path)).unwrap();
    fs::write(
        format!("{}/HEAD", &current_repo_initiation_path),
        "ref: refs/head/main\n",
    )
    .unwrap();
}

fn setup_revy(repository_name: Option<&str>) {
    let current_repo_initiation_path = fetch_path_for_repository_setup(repository_name);

    if check_if_directory_exists(&current_repo_initiation_path) {
        println!(
            "Reinitialized existing Reevy repository in {}",
            &current_repo_initiation_path
        );
        return;
    }

    initialize_repository(&current_repo_initiation_path);

    println!(
        "Initialized empty Reevy repository in {}",
        current_repo_initiation_path
    );
}

fn main() {
    let args = Args::parse();
    match args.commnds {
        Command::Init => {
            setup_revy(None);
        }
    }
}
