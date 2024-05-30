use std::path::Path;

use clap::{Parser, Subcommand};
mod blob;
mod command;
mod object;
mod tree;
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

    /// Create a hash of the object
    HashObject {
        /// The path to the file to hash
        file_path: String,

        /// Write the object into the object database
        #[arg(short, long)]
        write: bool,
    },
    LsTree {
        /// The hash of the object to display
        hash: String,
        /// List only filenames (instead of the "long" output), one per line. Cannot be combined with --object-only.
        #[arg(long)]
        name_only: bool,
    },

    WriteTree,
    CommitTree {
        /// The commit message
        #[arg(short, long)]
        message: String,

        /// The hash of the tree to commit
        hash: String,

        /// The hash of the parent commit
        #[arg(short, long)]
        parent: Option<String>,
    },
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.commnds {
        Command::Init { name } => {
            command::setup_revy(name.as_deref());
        }
        Command::CatFile { pretty_print, hash } => {
            if !pretty_print {
                todo!()
            }
            let object = object::Object::load_object_from_hash(&hash).unwrap();
            object.print_object();
        }
        Command::HashObject { file_path, write } => {
            let object = object::Object::create_blob(Path::new(&file_path).to_path_buf()).unwrap();
            if write {
                object.save_object();
            }
            println!("{}", object.hash);
        }
        Command::LsTree { name_only, hash } => {
            if !name_only {
                todo!()
            }

            let object = object::Object::load_object_from_hash(&hash).unwrap();
            match object.metadata {
                object::Metadata::Tree(tree) => tree.print_tree(),
                _ => panic!("Object is not a tree"),
            };
        }
        Command::WriteTree => {
            let object = object::Object::create_tree(None).unwrap();
            println!("{}", object.hash);
        }
        Command::CommitTree {
            hash,
            message,
            parent,
        } => {
            println!("{} {} {:?}", hash, message, parent);
            todo!();
        }
    }
    Ok(())
}
