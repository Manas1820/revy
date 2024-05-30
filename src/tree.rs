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
    /// Returns a string representation of the file mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            FileMode::RegularFile => "blob",
            FileMode::Directory => "tree",
            FileMode::SymbolicLink => "commit",
            FileMode::ExecutableFile => "commit",
        }
    }

    /// Converts a `u32` value to a `FileMode`.
    ///
    /// # Panics
    ///
    /// Panics if the mode is invalid.
    pub fn from_u32(mode: u32) -> FileMode {
        match mode {
            100644 => FileMode::RegularFile,
            100755 => FileMode::ExecutableFile,
            120000 => FileMode::SymbolicLink,
            040000 => FileMode::Directory,
            _ => panic!("Invalid file mode"),
        }
    }

    /// Returns the string representation of the `u32` file mode.
    pub fn as_u32_str(&self) -> &str {
        match self {
            FileMode::RegularFile => "100644",
            FileMode::Directory => "040000",
            FileMode::SymbolicLink => "120000",
            FileMode::ExecutableFile => "100755",
        }
    }
}

#[derive(Debug)]
pub struct Node {
    mode: FileMode,
    name: String,
    hash: String,
}

impl Node {
    /// Creates a new `Node` with the given mode, name, and hash.
    pub fn new(mode: FileMode, name: String, hash: String) -> Node {
        Node { mode, name, hash }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub data: Vec<Node>,
}

impl Tree {
    /// Creates a new `Tree` with the given nodes.
    pub fn new(data: Vec<Node>) -> Tree {
        Tree { data }
    }

    /// Parses a `Tree` from a buffered `ZlibDecoder` reader.
    ///
    /// # Arguments
    ///
    /// * `decoded_reader` - A buffered `ZlibDecoder` reader that reads the compressed tree data.
    ///
    /// # Errors
    ///
    /// Returns a `Result` containing the parsed `Tree` on success, or an error string if the tree cannot be parsed.
    ///
    /// The function reads until it encounters a null byte (0), indicating the end of a node's metadata.
    /// It then splits the metadata string to extract the mode and name of the node.
    /// The function also reads the following 20 bytes to get the SHA-1 hash of the node.
    /// If any of these operations fail, an error string is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use flate2::read::ZlibDecoder;
    ///
    /// let file = File::open("path/to/compressed/tree").unwrap();
    /// let decoder = ZlibDecoder::new(file);
    /// let reader = BufReader::new(decoder);
    ///
    /// match Tree::parse_tree(reader) {
    ///     Ok(tree) => println!("Parsed tree: {:?}", tree),
    ///     Err(err) => println!("Error parsing tree: {}", err),
    /// }
    /// ```
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

    /// Prints the names of the nodes in the tree.
    pub fn print_tree(&self) {
        for node in &self.data {
            println!("{}", node.name);
        }
    }

    /// Converts the tree to a byte vector.
    /// The byte vector contains the tree contents.
    ///
    /// Uses -
    ///
    /// - Is used to write the tree to a file.
    /// - And to calculate the content size
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

    /// Converts the tree to a string.
    pub fn as_str(&self) -> String {
        let mut tree_contents = String::new();
        for node in &self.data {
            let contents = format!("{} {}\0{}", node.mode.as_u32_str(), node.name, node.hash);
            tree_contents.push_str(&contents);
        }
        tree_contents
    }

    /// Prints a pretty representation of the tree.
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
}
