use std::{env, io};

mod fs_manager;
mod interactive;
mod utils;



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // if args contains --help or -h, print help
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        utils::print_help(&args);
    }

    let fp = if args.len() < 2 {
        match interactive::select_file() {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        args[1].clone()
    };


    let path = fp.clone();
    let file_manager = fs_manager::FileManager::new(fp);
    let contents = file_manager.read_file()?;

    if utils::is_json(&path) {
        if let Err(e) = file_manager.pretty_print_json(&contents) {
            eprintln!("Error: {}", e);
        }
    } else {
        println!("{}", contents);
    }

    Ok(())
}


