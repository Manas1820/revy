use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Blob {
    pub data: String,
}

impl Blob {
    pub fn new(data: String) -> Blob {
        Blob { data }
    }

    pub fn parse_blob(mut decoded_reader: BufReader<ZlibDecoder<File>>) -> Result<Blob, String> {
        let mut blob = Blob {
            data: String::new(),
        };

        let mut buffer = Vec::new();
        match decoded_reader.read_to_end(&mut buffer) {
            Ok(_size) => {}
            Err(_err) => {
                return Err("Failed to read object file while parsing blob".to_string());
            }
        }

        blob.data = String::from_utf8(buffer).unwrap();
        Ok(blob)
    }

    pub fn print_blob(&self) {
        println!("{}", self.data);
    }
}
