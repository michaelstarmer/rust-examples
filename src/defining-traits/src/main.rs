/*
 * 13 - Defining traits
 */

use rand::prelude::*;
struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("{}, {}", self.name, self.age);
    }
}

trait HasVoiceBox {
    
    // Speak
    fn speak(&self);

    // Check if can speak
    fn can_speak(&self) -> bool; // Returns a bool
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 2
        {
            return true;
        }

        return false;
    }
}

fn generate_persons(len: u32) -> Vec<Person> {
    let mut vec_persons = Vec::new();

    
    for it in 1..=len {
        let pid = rand::thread_rng().gen_range(1, 70);
        let new_person = Person {
            name: format!("Person {}", &pid),
            age: 2,
        };
        println!("new person: {}", &new_person.to_string());
        vec_persons.push(new_person);
        
    }
    return vec_persons;
}

fn main() {

    let person = Person {
        name: String::from("Egil"),
        age: 4
    };

    let vec1 = vec!(generate_persons(5));

    for i in 0..vec1.len() {
        println!("{}", vec1[i].to_string());
    }
    
}
 