use crate::interactive;
use std::{io, process};

// Print help message, if the user asks for it
pub fn handle_flags(args: &[String]) -> String {
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help(&args[0]);
        std::process::exit(0);
    }

    get_file_path(args).unwrap_or_else(|e| {
        eprintln!("Error getting file path: {}", e);
        process::exit(1);
    })
}

/// Documentation string
fn print_help(prog_name: &str) {
    println!("Usage: {} [<file_path>]", prog_name);
    println!("If no file path is provided, the program will present an interactive dialogue.");
}

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
}
