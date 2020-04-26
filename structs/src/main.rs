/*
 * Tutorial 02
 */

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    // init
    let mut bg = Color { red: 255, green: 70, blue: 15 };

    // edit
    bg.blue = 255;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
    
}