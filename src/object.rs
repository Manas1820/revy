use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use crate::blob;
use crate::tree;
use crate::tree::Node;
use crate::utils;

#[derive(Debug)]
pub enum ObjectType {
    Tree,
    Blob,
}

impl ObjectType {
    /// Returns the string representation of the `ObjectType`.
    ///
    /// # Returns
    /// A string slice that represents the type of the object (`"tree"` or `"blob"`).
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
        }
    }

    /// Creates an `ObjectType` from a string.
    ///
    /// # Arguments
    /// * `mode` - A string slice representing the mode of the object.
    ///
    /// # Returns
    /// An `Option` containing the `ObjectType` if the string matches a known type, or `None` otherwise.
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
    pub hash: String,
    pub size: usize,
    pub metadata: Metadata,
}

#[allow(dead_code)]
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

    pub fn new(kind: ObjectType, metadata: Metadata) -> Result<Object, ()> {
        let content = match &metadata {
            Metadata::Tree(tree) => tree.as_str().to_string(),
            Metadata::Blob(blob) => blob.as_str().to_string(),
        };

        let size = match &metadata {
            Metadata::Tree(tree) => tree.as_bytes().len(),
            Metadata::Blob(blob) => blob.as_bytes().len(),
        };

        let object = Object {
            kind,
            hash: utils::generate_sha1(&content),
            size: size as usize,
            metadata,
        };

        Ok(object)
    }

    pub fn save_object(&self) {
        let mut current_repo_directory = std::path::PathBuf::from(".");
        current_repo_directory.push(format!(
            ".revy/objects/{}/{}",
            &self.hash[..2],
            &self.hash[2..]
        ));

        if let Some(parent) = current_repo_directory.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                println!("Failed to create directories: {:?}", err);
                return;
            }
        }

        let object_file = match File::create(&current_repo_directory) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to create object file: {:?}", current_repo_directory);
                return;
            }
        };

        let mut file_data = Vec::new();

        let mut encoder = ZlibEncoder::new(object_file, Compression::default());
        let header = format!("{} {}\0", self.kind.as_str(), self.size);

        file_data.extend(header.as_bytes());
        file_data.extend(self.metadata_as_bytes());

        encoder.write_all(&file_data).unwrap();
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

        let size = match header[1].parse::<usize>() {
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
                let blob = match blob::Blob::from_file(decoded_reader) {
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
            hash: hash.to_string(),
            size,
            metadata,
        })
    }

    pub fn create_tree(path: Option<std::path::PathBuf>) -> Result<Object, ()> {
        let mut tree_nodes: Vec<Node> = Vec::new();
        let tree_curr_path = match path {
            Some(path) => path,
            None => std::path::Path::new(".").to_path_buf(),
        };
        let all_paths = read_dir(tree_curr_path).unwrap();

        let excluded_paths = utils::fetch_excluded_paths();
        let entries = all_paths.filter_map(Result::ok).filter(|entry| {
            let path = entry.path();
            !utils::should_ignore(&path, &excluded_paths)
        });

        for entry in entries {
            let curr_path = &entry.path();
            let curr_path_file = curr_path.file_name().unwrap().to_str().unwrap();
            if curr_path.is_dir() {
                let sub_tree_object = Object::create_tree(Some(curr_path.to_path_buf())).unwrap();
                tree_nodes.push(Node::new(
                    tree::FileMode::Directory,
                    curr_path_file.to_string(),
                    sub_tree_object.hash,
                ));
            } else {
                let blob_object = Object::create_blob(curr_path.to_path_buf()).unwrap();
                blob_object.save_object();
                tree_nodes.push(Node::new(
                    tree::FileMode::RegularFile,
                    curr_path_file.to_string(),
                    blob_object.hash,
                ));
            }
        }

        let tree = tree::Tree::new(tree_nodes);
        let object = Object::new(ObjectType::Tree, Metadata::Tree(tree)).unwrap();
        object.save_object();

        Ok(object)
    }

    pub fn create_blob(path: std::path::PathBuf) -> Result<Object, String> {
        let blob_contents = match std::fs::read(&path) {
            Ok(contents) => contents,
            Err(_err) => {
                return Err("Failed to read file".to_string());
            }
        };

        let blob_contents_str = match String::from_utf8(blob_contents) {
            Ok(contents) => contents,
            Err(_err) => {
                return Err("Failed to convert file contents to string".to_string());
            }
        };

        let blob = blob::Blob::new(blob_contents_str);
        let object = Object::new(ObjectType::Blob, Metadata::Blob(blob)).unwrap();

        Ok(object)
    }

    pub fn print_object(&self) {
        match &self.metadata {
            Metadata::Tree(tree) => {
                tree.print_tree();
            }
            Metadata::Blob(blob) => {
                blob.print_blob();
            }
        }
    }

    fn metadata_as_str(&self) -> String {
        let metadata = match &self.metadata {
            Metadata::Tree(tree) => tree.as_str().to_string(),
            Metadata::Blob(blob) => blob.as_str().to_string(),
        };
        metadata
    }

    fn metadata_as_bytes(&self) -> Vec<u8> {
        let metadata = match &self.metadata {
            Metadata::Tree(tree) => tree.as_bytes(),
            Metadata::Blob(blob) => blob.as_bytes(),
        };
        metadata
    }
}
