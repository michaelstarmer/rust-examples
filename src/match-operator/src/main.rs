/*
 * Tutorial 14: Match operator
 * 
 * Like a switch statement.
 */

use log::{info, trace, warn};

extern crate simple_logger;

fn main() {
    simple_logger::init().unwrap();
    let number = 11;

    println!("LOG: println");
    info!("LOG: info");
    trace!("LOG: trace");
    warn!("LOG: warn");

    match number {
        1 => println!("It is 1"),
        2...20 => {
            println!("It's a number between 2 and 20");
        },
        _ => println!("No match"),
    }
}
