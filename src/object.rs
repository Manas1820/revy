use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[path = "utils.rs"]
mod utils;

#[path = "tree.rs"]
mod tree;

#[path = "blob.rs"]
mod blob;

#[derive(Debug)]
pub enum ObjectType {
    Tree,
    Blob,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
        }
    }

    pub fn from_str(mode: &str) -> Option<ObjectType> {
        match mode {
            "tree" => Some(ObjectType::Tree),
            "blob" => Some(ObjectType::Blob),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Metadata {
    Tree(tree::Tree),
    Blob(blob::Blob),
}

#[derive(Debug)]
pub struct Object {
    pub kind: ObjectType,
    pub size: u32,
    pub metadata: Metadata,
}

impl Object {
    fn load_file_from_hash(hash: &str) -> Result<File, String> {
        let current_repo_directory = utils::fetch_path_for_repository(None);
        let current_working_directory = format!(
            "{}/objects/{}/{}",
            current_repo_directory,
            &hash[..2],
            &hash[2..]
        );

        let object_file = match File::open(&current_working_directory) {
            Ok(file) => file,
            Err(_err) => {
                return Err(format!(
                    "Failed to open object file: {}",
                    &current_working_directory
                ));
            }
        };

        Ok(object_file)
    }

    pub fn load_object_from_hash(hash: &str) -> Result<Object, String> {
        let object_file = match Object::load_file_from_hash(hash) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };

        let decoder = ZlibDecoder::new(object_file);
        let mut decoded_reader = BufReader::new(decoder);

        let mut header_contents = Vec::new();
        match decoded_reader.read_until(0, &mut header_contents) {
            Ok(_size) => {}
            Err(_err) => {
                return Err("Failed to read object file, invalid header".to_string());
            }
        }

        let header_parts: Result<Vec<&str>, String> =
            match std::ffi::CStr::from_bytes_until_nul(&header_contents) {
                Ok(header) => {
                    let header_str = header.to_str().unwrap();
                    let header_parts: Vec<&str> = header_str.split_whitespace().collect();
                    Ok(header_parts)
                }
                Err(_err) => {
                    return Err("Failed to read object file, invalid header".to_string());
                }
            };

        let header = match header_parts {
            Ok(parts) => parts,
            Err(err) => {
                return Err(err);
            }
        };

        let kind = match ObjectType::from_str(header[0]) {
            Some(object_type) => object_type,
            None => {
                return Err(format!("Malformed object file size in {}", &hash));
            }
        };

        let size = match header[1].parse::<u32>() {
            Ok(size) => size,
            Err(_err) => {
                return Err(format!("Malformed object file size in {}", &hash));
            }
        };

        let metadata = match kind {
            ObjectType::Tree => {
                let tree = match tree::Tree::parse_tree(decoded_reader) {
                    Ok(tree) => tree,
                    Err(err) => {
                        return Err(err);
                    }
                };
                Metadata::Tree(tree)
            }
            ObjectType::Blob => {
                let blob = match blob::Blob::parse_blob(decoded_reader) {
                    Ok(blob) => blob,
                    Err(err) => {
                        return Err(err);
                    }
                };
                Metadata::Blob(blob)
            }
        };

        Ok(Object {
            kind,
            size,
            metadata,
        })
    }
}
