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

    pub fn read_file(&self) -> io::Result<String> {
        let file = File::open(&self.path)?;

        // Shadowing to handle the error
        let file = handle_io_errors(&file)?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn pretty_print_json(contents: &str) -> Result<(), serde_json::Error> {
        let v: Value = serde_json::from_str(contents)?;
        println!("{}", serde_json::to_string_pretty(&v)?);
        Ok(())
    }
}

/// Handle IO errors
/// Actually unnecessary, but it's here to let me learn how to handle different types of errors
fn handle_io_errors(f: &File) -> Result<File, io::Error> {
    let f = match f {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("FAILURE: \nFile not found, did you spell it correctly?\n");
                Err(error)
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("FAILURE: \nPermission denied, do you have the right permissions?\n");
                Err(error)
            }
            _ => {
                println!("Some other error occurred: {:?}", error);
                Err(error)
            }
        },
    };
}
