use std::{env, io};

mod fs_manager;
mod interactive;
mod utils;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // if args contains --help or -h, print help
    let fp = utils::handle_flags(&args);

    process_file(fp)
}

fn process_file(file_path: String) -> io::Result<()> {
    let file_manager = fs_manager::FileManager::new(file_path.clone());
    let contents = file_manager.read_file()?;

    if utils::is_json(&file_path) {
        file_manager.pretty_print_json(&contents).map_err(|e| {
            eprintln!("Error printing JSON: {}", e);
            io::Error::new(io::ErrorKind::Other, "JSON processing failed")
        })
    } else {
        println!("{}", contents);
        Ok(())
    }
}
