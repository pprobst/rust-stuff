/*
 * The open static method can be used to open a file in read-only mode.
 * A File owns a resource, the file descriptor and takes care of closing the file when it is droped.
 */

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file.
    let path = Path::new("hello.txt");
    let display = path.display();
    println!("Path: {}", display); // hello.txt

    // Open the path in read-only mode, returns 'io::Result<File>'.
    let mut file = match File::open(&path) {
        // The 'description' method of 'io::Error' returns a string that describes the error.
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns 'io::Result<usize>'.
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // 'file' goes out of scope, and the "hello.txt" file gets closed.
}
