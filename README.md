# Rust file reader

This is a small example of how to read a file in Rust.
The project is my solution to the first assignment of the course [Rust Programming Specialization](Part of the Coursera specialization [Rust Programming Specialization](https://www.coursera.org/specializations/rust-programming) on Coursera
by Alfredo Deza at Duke University.

See [ASSIGNMENT.md](ASSIGNMENT.md) for the original assignment.

### Learning objectives

- Read a file in Rust 
- Handle errors in Rust
- Familiarize with the `Result` type, `match` and `unwrap` in Rust
- Have fun with Rust :)

I've called it json_reader, since I wanted to experiment with the `serde_json` crate
and have a json pretty print of the file content.

### Crates I've experimented with

- `serde_json` for pretty printing the json content of the file ([v.1.0.116](https://docs.rs/serde_json/1.0.116/serde_json/))
- `dialoguer` for user interaction ([v.0.11.0](https://docs.rs/dialoguer/0.11.0/dialoguer/))

## Structure

~~~sh
.
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md 
├── cliff.toml # Configuration file for the Changelog
├── LICENSE # MIT License
├── Makefile # Makefile for building and running the project
├── README.md 
├── src # Source code
│   ├── fs_manager.rs # File system manager (open and read files)
│   ├── interactive.rs # User interaction with dialoguer
│   ├── main.rs # Main entry point
│   └── utils.rs # Utility functions
├── target # Rust build directory
└── test.json # Test file for json pretty print

~~~
