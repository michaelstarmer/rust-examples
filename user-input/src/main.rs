/*
 * Tutorial 15: User Input
 * 
 * Reading input from the console.
 */

use std::io;

use log::{info, trace, warn};

extern crate simple_logger;

fn init_application() {
    let mut header = String::new();
    let title = String::from("Application is running.");

    for _token in title.chars() {
        header.push_str("=");
    }

    println!("\n\n");
    println!("{}==", &header);
    println!("\n{}\n", &title);
    println!("{}==", &header);
}

fn main() {
    simple_logger::init().unwrap();
    init_application();

    let mut input = String::new();
    info!("Say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => { // b or _
            info!("You said: {}", input.to_uppercase());
        },
        Err(e) => {
            warn!("Something went wrong: {}", e);
        }
    }
}
