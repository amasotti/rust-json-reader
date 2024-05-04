use crate::interactive;
use std::{io, process};

/// Handle the flags passed to the program
///
/// If the user asks for help, print the help message and exit.
/// If the user provides a file path, return it.
/// If the user provides no file path, present an interactive dialogue.
pub fn parse_user_choice(args: &[String]) -> io::Result<Option<String>> {
    // If the user asks for help, print the help message and exit.
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help(&args[0]);
        return Ok(None); // Signal to exit with success.
    }

    get_file_path(args).map(Some)
}

/// Documentation string
fn print_help(prog_name: &str) {
    println!("Usage: {} [<file_path>]", prog_name);
    println!("If no file path is provided, the program will present an interactive dialogue.");
}

/// Get the file path from the arguments or present an interactive dialogue
pub fn get_file_path(args: &[String]) -> io::Result<String> {
    if args.len() < 2 {
        interactive::select_file().map_err(|e| {
            eprintln!("Error selecting file: {}", e);
            process::exit(1);
        })
    } else {
        Ok(args[1].clone())
    }
}

/// Check if the file is a JSON file
pub fn is_json(file_path: &str) -> bool {
    file_path.ends_with(".json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_json() {
        assert_eq!(is_json("file.json"), true);
        assert_eq!(is_json("file.txt"), false);
    }

    #[test]
    fn test_doc_string() {
        assert_eq!(print_help("test_program"), ());
    }

    #[test]
    fn test_parse_user_choice() {
        let args = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(parse_user_choice(&args).unwrap(), Some("b".to_string()));
    }
}
