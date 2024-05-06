use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::prelude::*;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commnds: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Intialize the revy repository
    Init {
        /// The name of the new repository
        name: Option<String>,
    },
    /// Print the contents of a file
    CatFile {
        /// Definne if the output should be pretty
        #[arg(short, long)]
        pretty_print: bool,
        /// The hash of the object to display
        hash: String,
    },
}

fn setup_revy(repository_name: Option<&str>) {
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

fn read_from_objects(hash: &str) {
    // TODO: implement shortest hash lookup
    /*
     Objectives:
        - Read the contents of the blob object file from the .git/objects directory
        - Decompress the contents using Zlib
        - Extract the actual "content" from the decompressed data
        - Print the content to stdout
    */

    let current_repo_directory = utils::fetch_path_for_repository(None);

    let current_working_directory = format!(
        "{}/objects/{}/{}",
        current_repo_directory,
        &hash[..2],
        &hash[2..]
    );

    let object_file = match fs::File::open(&current_working_directory) {
        Ok(file) => file,
        Err(_err) => {
            eprintln!("Failed to open object file: {}", &current_working_directory);
            return;
        }
    };

    let mut decoder = ZlibDecoder::new(object_file);
    let mut contents = String::new();
    if let Err(_err) = decoder.read_to_string(&mut contents) {
        eprintln!("Failed to read object file: {}", &current_working_directory);
        return;
    }

    if let Some(index) = contents.find('\0') {
        let data = contents.split_off(index + 1);
        print!("{}", data);
    } else {
        print!("Malformed object found at {}", &current_working_directory);
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.commnds {
        Command::Init { name } => {
            setup_revy(name.as_deref());
        }
        Command::CatFile { pretty_print, hash } => {
            if !pretty_print {
                todo!()
            }
            read_from_objects(&hash);
        }
    }

    Ok(())
}
