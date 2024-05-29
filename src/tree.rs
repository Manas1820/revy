use std::fs::File;
use std::hash::{self, Hash};
use std::io::{BufRead, BufReader};

use flate2::read::ZlibDecoder;

#[derive(Debug)]
pub enum FileMode {
    RegularFile = 100644,
    ExecutableFile = 100755,
    SymbolicLink = 120000,
    Directory = 040000,
}

impl FileMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileMode::RegularFile => "blob",
            FileMode::Directory => "tree",
            FileMode::SymbolicLink => "commit",
            FileMode::ExecutableFile => "commit",
        }
    }

    pub fn from_u32(mode: u32) -> FileMode {
        match mode {
            100644 => FileMode::RegularFile,
            100755 => FileMode::ExecutableFile,
            120000 => FileMode::SymbolicLink,
            040000 => FileMode::Directory,
            _ => panic!("Invalid file mode"),
        }
    }

    pub fn as_u32_str(&self) -> &str {
        match self {
            FileMode::RegularFile => "100644",
            FileMode::Directory => "040000",
            FileMode::SymbolicLink => "120000",
            FileMode::ExecutableFile => "100755",
        }
    }
}

// Node: A node in a tree is a file or a directory. It has the following fields:
/*
    - <mode> is the mode of the file/directory (check the previous section for valid values)
    - <name> is the name of the file/directory
    - <20_byte_sha> is the 20-byte SHA-1 hash of the blob/tree (this is not in hexadecimal format)
*/

#[derive(Debug)]
pub struct Node {
    mode: FileMode,
    name: String,
    hash: String,
}

impl Node {
    pub fn new(mode: FileMode, name: String, hash: String) -> Node {
        Node { mode, name, hash }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub data: Vec<Node>,
}

impl Tree {
    pub fn new(data: Vec<Node>) -> Tree {
        Tree { data }
    }

    pub fn parse_tree(mut decoded_reader: BufReader<ZlibDecoder<File>>) -> Result<Tree, String> {
        let mut tree = Tree { data: Vec::new() };

        while !decoded_reader.buffer().is_empty() {
            let mut node_data = Vec::new();
            match decoded_reader.read_until(0, &mut node_data) {
                Ok(_size) => {}
                Err(_err) => {
                    return Err("Failed to read object file while parsing tree".to_string());
                }
            }

            let node_str = std::ffi::CStr::from_bytes_until_nul(&node_data)
                .unwrap()
                .to_str()
                .unwrap();

            let node_parts: Vec<&str> = node_str.split_whitespace().collect();

            if node_parts.len() != 2 {
                return Err("Failed to read object file while parsing tree".to_string());
            }

            let mode = match node_parts[0].parse::<u32>() {
                Ok(mode) => mode,
                Err(_err) => {
                    return Err("Failed to read object file while parsing tree".to_string());
                }
            };

            let hash_buffer = decoded_reader.buffer()[0..20].to_vec();
            decoded_reader.consume(20);
            let hash = hex::encode(hash_buffer);

            tree.data.push(Node {
                mode: FileMode::from_u32(mode),
                name: node_parts[1].to_string(),
                hash,
            });
        }

        Ok(tree)
    }

    pub fn print_tree(&self) {
        for node in &self.data {
            println!("{}", node.name);
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut tree_contents = Vec::new();
        for node in &self.data {
            let contents = format!("{} {}\0", node.mode.as_u32_str(), node.name);
            tree_contents.extend(contents.as_bytes());

            let hash = hex::decode(&node.hash).unwrap();
            tree_contents.extend(&hash);
        }
        tree_contents
    }

    pub fn as_str(&self) -> String {
        let mut tree_contents = String::new();
        for node in &self.data {
            let contents = format!("{} {}\0{}", node.mode.as_u32_str(), node.name, node.hash);
            tree_contents.push_str(&contents);
        }
        tree_contents
    }

    pub fn print_pretty_tree(&self) {
        for node in &self.data {
            println!(
                "{} {} {} {}",
                node.mode.as_u32_str(),
                node.mode.as_str(),
                node.hash,
                node.name
            );
        }
    }

    // pub fn generate_tree(&self) -> Result<String, String> {
    //     let mut tree_data = String::new();
    //     for node in &self.data {
    //         tree_data.push_str(&format!("{:?} {}\0{}", node.mode, node.name, node.hash));
    //     }
    //     let final_tree = format!("{}{}\0", "tree", tree_data);
    //     return Ok(final_tree);
    // }
}
