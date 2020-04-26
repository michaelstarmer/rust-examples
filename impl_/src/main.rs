/*
 * Tutorial 06
 */

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 5 };
    rect1.print_description();
    println!("Rectangle 1 is a square: {}", rect1.is_square());

    let rect2 = Rectangle { width: 5, height: 5 };
    rect2.print_description();
    println!("Rectangle 2 is a square: {}", rect2.is_square());
    
}
