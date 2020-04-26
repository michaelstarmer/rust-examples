/*
 * Tutorial 14: Match operator
 * 
 * Like a switch statement.
 */

fn main() {
    let number = 2;

    println!("First time: {}", number);
    println!("second time: {}", &number);
    
    match number {
        1 => println!("It is 1"),
        2 => println!("This is 2"),
        _ => println!("No match"),
    }
}
