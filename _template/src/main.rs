/*
 * $project-title
 * 
 * Run w/logging: RUST_LOG=trace cargo run
 * Run default:   cargo run
 */

mod logger;
#[macro_use] extern crate log;

fn main() {
   logger::init_application();
   

}
