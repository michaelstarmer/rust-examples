/*
 * multiple-src-files
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */

mod logger;

#[macro_use] extern crate log;


fn main() {
   logger::init_application();
   
   let name = String::from("Hilda");

   info!("Character at index 3: {}", match name.chars().nth(3) {
      Some(c) => c.to_string(),
      None => "No character at index 3".to_string(),
   });

   info!("Occupation is {}", match get_occupation("Felix") {
      Some(o) => o,
      None => "No occupation found",
   });
}

fn get_occupation(name: &str) -> Option<&str> {
   match name {
      "Andrew" => Some("Car Mechanic"),
      "Felix" => Some("Gardener"),
      _ => None
   }
}