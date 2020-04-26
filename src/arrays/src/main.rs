/*
 * Tutorial 04
 */

fn main() {
    let numbers = [ 11, 12, 13, 14, 15 ];
    
    // With default
    // let numbers = [2;400];

    // With type and len
    // let numbers: [i32; 5] = [ 11, 12, 13, 14, 15 ];

    for i in 0..numbers.len() {
        println!("Index {}: {}", i, numbers[i]);
    }

    for n in numbers.iter() {
        println!("Number {}", n);
    }
}
