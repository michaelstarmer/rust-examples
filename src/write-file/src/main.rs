/*
 * 12 - Writing to a file
 */

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("Could not create file");

    file.write_all(b"This is the result.")
        .expect("Cannot write to file.");
}
