# Lab Assignment: Create a file reader application

In this lab, you will enhance a file reader application in Rust.


## Learning Objectives:

- Understand how to read and process files in Rust using the standard library.
- Practice error handling techniques, such as matching on different error cases.
- Learn how to organize code and structure a Rust application.

Start with this sample code:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
}
```


## Concept covered


- __Introduction to Rust__
- __Variable assignment and immutability__ 
- __Basics of control flow__
- __Function Basics__
- __Error handling__
- __File processing__

_By completing this lab, you will gain practical experience in Rust by extending an existing 
file reader application. 
You will develop skills in file I/O, error handling, and some basic code organization, 
utilizing the concepts introduced in the lessons for this week._
