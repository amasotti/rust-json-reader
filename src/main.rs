use std::env;
use std::fs::File;
use std::io::{self, Read, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("{}", contents);
    Ok(())
}
