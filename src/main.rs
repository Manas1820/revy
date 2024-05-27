use clap::{Parser, Subcommand};
mod command;
mod object;
mod tree;

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
    },
    LsTree {
        /// The hash of the object to display
        hash: String,
        /// List only filenames (instead of the "long" output), one per line. Cannot be combined with --object-only.
        #[arg(long)]
        name_only: bool,
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
            match object.metadata {
                object::Metadata::Blob(blob) => blob.print_blob(),
                object::Metadata::Tree(tree) => tree.print_pretty_tree(),
                _ => panic!("Object is not a blob"),
            };
        }
        Command::HashObject { file_path } => match command::generate_sha1_hash(file_path) {
            Ok(hash) => println!("{}", hash),
            Err(err) => eprintln!("{}", err),
        },
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
    }
    Ok(())
}
