/*
 * Tutorial 07
 */

fn main() {
    let mut my_string = String::from("This is");

    println!("Length: {}", my_string.len());
    println!("isEmpty? {}", my_string.is_empty());
    println!("Contains 'zz'? {}", my_string.contains("zz"));
    
    /* Append */
    my_string.push_str(" a string.");
    println!("New string: {}", &my_string);
    
    /* Split */
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    /* Replace */
    {
        let my_string = String::from("Rust is fantastic!");
        
        println!("Replace success: {}", my_string.replace("fantastic", "great"));
    }

    /* Lines */
    {
        let my_string = String::from("The weather is \nnice\noutside mate!");

        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    /* Split */
    {
        let my_string = String::from("There+are+not+enough+birds+in+the+sky.");
        let tokens: Vec<&str> = my_string.split("+").collect(); // Split() returns iterator, but collect() will transform that to a Vector of String slices

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);

        for it in tokens.iter() {
            println!("{}", it);
        }
    }

    /* Trim */
    {
        println!("\n\n============= TRIM =============\n");
        let my_string = String::from("      My name is Al\n    \r\t");

        println!("Before: {}", my_string);
        println!("After:\t{}", my_string.trim());
    }

    /* Chars */
    {
        println!("\n\n============= CHARS =============\n");
        let my_string = String::from("one pizza on the table");
        
        println!("{}", my_string);

        // Get char at index
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4"),
        }
    }
}
