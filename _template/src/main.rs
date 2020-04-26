/*
 * $project-title
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */

use rand::prelude::*;

 extern crate pretty_env_logger;
 #[macro_use] extern crate log;
 
 fn init_application() {
    let mut header = String::new();
    let title = String::from("Application is running. Valid log types below.");

    for _token in title.chars() {
        header.push_str("=");
    }

    println!("\n\n");
    println!("{}==", &header);
    println!("\n{}\n", &title);
    trace!("message");
    debug!("message");
    info!("message");
    warn!("message");
    error!("message");
    println!("\n{}==", &header);
    
    println!("");
 }
 
 fn main() {
    pretty_env_logger::init();
    init_application();

    let rnd_nr = rand::thread_rng().gen_range(1, 101);
    info!("Generated {}\n", rnd_nr);
    
    println!("\n");
 }
 