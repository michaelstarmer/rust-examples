/*
 * Tutorial 09
 * Vectors
 */

fn main() {
    // let my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4];
    println!("Vector length: {}", my_vector.len());

    my_vector.push(40);
    println!("Vector length: {}", my_vector.len());

    my_vector.remove(1);
    println!("Vector length: {}", my_vector.len());

    println!("\n-- START --");
    for number in my_vector.iter() {
        println!("{}", number);
    }
    println!("-- END --");
    

}
