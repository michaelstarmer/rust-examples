/*
 * Tutorial 03
 */

struct Color(u8, u8, u8);

fn main() {
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    println!("{}, {}, {}", red.0, red.1, red.2);
}
