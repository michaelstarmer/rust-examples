/*
 * 11 - Command line arguments
 */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        println!("Arg {}: {}", i, args[i]);
        
    }
}
 