/*
 * multiple-src-files
 * 
 * To enable all logging: RUST_LOG=trace cargo run
 */

 use regex::Regex;
 
 mod logger;
 #[macro_use] extern crate log;
 extern crate regex;


fn main() {
   logger::init_application();
   
   let re = Regex::new(r"(\w{5})").unwrap();
   let text = "dcode";

   for capture in re.captures_iter(text) {
      info!("Captures - 1: {}", &capture);
   }
   // match re.captures(text) {
   //    Some(caps) => info!("Captures: {}", &caps[0].unwrap().as_str()),
   //    None => debug!("No captures."),
   // }

}
