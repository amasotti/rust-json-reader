use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use serde_json::Value;

/// Dummy struct for the file manager
pub struct FileManager {
    pub path: String,
}

/// Implementation of the FileManager struct
///
/// new: Constructor for the FileManager struct
///
/// read_file: Reads the file and returns the contents as a string
///
/// pretty_print_json: Pretty prints the JSON content
impl FileManager {
    pub fn new(path: String) -> FileManager {
        FileManager {
            path: path.to_string(),
        }
    }

    /// Reads the file at the stored path and returns the contents as a String
    pub fn read_file(&self) -> io::Result<String> {
        let file = File::open(&self.path);

        let file = match handle_io_errors(file) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn pretty_print_json(&self, contents: &str) -> Result<(), serde_json::Error> {
        let v: Value = serde_json::from_str(contents)?;
        println!("{}", serde_json::to_string_pretty(&v)?);
        Ok(())
    }
}

/// Handle IO errors
/// Actually unnecessary, but it's here to let me learn how to handle different types of errors
fn handle_io_errors(result: io::Result<File>) -> io::Result<File> {
    match result {
        Ok(file) => Ok(file),
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("Error: File not found. Did you spell it correctly?");
                }
                io::ErrorKind::PermissionDenied => {
                    println!("Error: Permission denied. Do you have the right permissions?");
                }
                _ => {
                    println!("Error: An unexpected error occurred: {:?}", error);
                }
            }
            Err(error)
        }
    }
}
