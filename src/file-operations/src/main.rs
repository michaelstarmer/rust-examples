/*
 * hangman
 * 
 * Run w/logging: RUST_LOG=trace cargo run
 * Run default:   cargo run
 */

#[macro_use]
extern crate log;
extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

mod logger;

fn main()
{
   logger::init_application();
   
   let selected_word = select_word();
}

fn select_word() -> String
{
   // Open file
   let mut file = File::open("words.txt")
      .expect("Could not open file!");

   // Load content
   let mut file_contents = String::new();

   file.read_to_string(&mut file_contents)
      .expect("Error reading file!");

   // Get words
   let available_words: Vec<&str> = file_contents.trim().split(',').collect();

   for word in available_words.iter() {
      info!("{}", word);
   }

   return String::from("String return");
}