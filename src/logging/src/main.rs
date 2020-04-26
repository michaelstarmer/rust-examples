/*
 * Tutorial 16: Logging
 * 
 * Reading input from the console.
 * To enable all logging: RUST_LOG=trace cargo run
 */

 use std::io;

//  use log::{info, trace, warn};
 
 extern crate pretty_env_logger;
 #[macro_use] extern crate log;
 
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
    pretty_env_logger::init();
    init_application();

    println!("\nLogging types preview:\n");

    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
    println!("\n");

    let mut input = String::new();
    
     
 }
 