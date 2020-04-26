/*
 * 10 - Reading a file
 */

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("Dockerfile").expect("Can't open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Can't read the file");

    println!("File content:\n\n{}", content);
    
}
