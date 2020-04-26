/*
 * Tutorial 07
 */

fn main() {
    let mut my_string = String::from("Wazzup");

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Length: {}", my_string.len());
    println!("isEmpty? {}", my_string.is_empty());
    println!("Contains 'zz'? {}", my_string.contains("zz"));
    
    my_string.push_str(" ma niggah");

    println!("New string: {}", &my_string);
    

}
