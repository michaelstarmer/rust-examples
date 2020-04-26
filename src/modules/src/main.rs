/*
 * multiple-src-files
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */

mod logger;
mod cmod {

   // private
   fn showData() {
      info!("Log from private function.")
   }

   // public
   pub fn print_message() {
      info!("Module initialized");
      showData();
   }

   pub mod helper {
      pub fn print_message() {
         info!("Helper function")
      }
   }
}

#[macro_use] extern crate log;


fn main() {
   
   logger::init_application();
   cmod::print_message();
   cmod::helper::print_message();
   
}
