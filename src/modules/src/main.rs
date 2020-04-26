/*
 * multiple-src-files
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */

use rand::prelude::*;
mod logger;

#[macro_use] extern crate log;


fn main() {
   
   logger::init_application();
   
}
