/*
 * functions
 * 
 * Run w/logging: RUST_LOG=trace cargo run
 * Run default:   cargo run
 */

mod logger;
#[macro_use] extern crate log;

fn main() {
   logger::init_application();
   
   basic_function();
   
   let return_type = return_type_function();
   if (return_type) {
      info!("return_type = TRUE");
   } else {
      info!("return_type = FALSE");
   }

   let blue = Color { red: 0, green: 0, blue: 255 };
   param_function(&blue);
}

struct Color {
   red: u8,
   green: u8,
   blue: u8,
}

/// Basic function
fn basic_function()
{
   info!("A basic function.");
}

/// Function with return type
/// @return {bool}
fn return_type_function() -> bool
{
   info!("A simple function with return type (bool)");

   return true;
}

/// Function with parameter
/// @param {&struct}  color
/// @return {void}
fn param_function(c: &Color)
{
   info!("Color");
   info!("RGB({},{},{})\n", c.red, c.green, c.blue);
}