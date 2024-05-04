pub fn print_help(args: &Vec<String>) {
        println!("Usage: {} [<file_path>]", args[0]);
        println!("If no file path is provided, the program will present an interactive dialogue.");
        std::process::exit(0);
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


