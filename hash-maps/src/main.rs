/*
 * Tutorial 16: Logging
 * 
 * Reading input from the console.
 * To enable all logging: RUST_LOG=trace cargo run
 */

 use std::collections::HashMap;

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

    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 84);
    marks.insert("UI Design", 88);
    marks.insert("Networks", 98);

    info!("Classes: {}", marks.len());
    
    // Get mapped value
    match marks.get("Web Development") {
        Some(mark) => info!("You got {} for Web Development", mark),
        None => warn!("You did not study Web Development"),
    }

    marks.remove("UI Design");
    info!("Class removed. Number of studies now {}", marks.len());

    println!("\n");

    for (subject, mark) in &marks {
        info!("For class {} you got {}%", subject.to_uppercase(), mark);
    }
    
    println!("\n");
 }
 