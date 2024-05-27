use std::fs;

#[path = "utils.rs"]
mod utils;

pub fn setup_revy(repository_name: Option<&str>) {
    let current_repo_initiation_path = utils::fetch_path_for_repository(repository_name);

    if utils::check_if_directory_exists(&current_repo_initiation_path) {
        println!(
            "Reinitialized existing Revy repository in {}",
            &current_repo_initiation_path
        );
        return;
    }

    utils::initialize_repository(&current_repo_initiation_path);

    println!(
        "Initialized empty Revy repository in {}",
        current_repo_initiation_path
    );
}

pub fn generate_sha1_hash(file_path: String) -> Result<String, String> {
    // Objectives:

    /*
        - Check if the file exists
        - Read the contents of the file
        - Get the file size
        - Create a string with the following format: "blob {file_size}\0{file_contents}"
        - Generate a hash of the file contents
        - Return the hash
    */

    let is_a_valid_file = utils::check_if_directory_exists(&file_path);
    if !is_a_valid_file {
        return Err(format!(
            "fatal: could not open '{}' for reading: No such file or directory",
            file_path
        ));
    }

    let file_contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(err.to_string()),
    };

    let file_meta_data = match fs::metadata(&file_path) {
        Ok(meta_data) => meta_data,
        Err(err) => return Err(err.to_string()),
    };

    let data_to_write = format!("blob {}\0{}", file_meta_data.len(), file_contents);
    let generated_hash = utils::generate_sha1(&data_to_write);

    Ok(generated_hash)
}
