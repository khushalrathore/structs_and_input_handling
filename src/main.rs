use std::io::{self, Write};

struct Rectangle {
    length: f32,
    breadth: f32,
}

impl Rectangle {
    fn new(length: f32, breadth: f32) -> Rectangle {
        Rectangle { length, breadth }
    }

    fn area(&self) -> f32 {
        self.length * self.breadth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.breadth > other.breadth
    }
}

fn main() {
    
    print!("Enter length & breadth (space-separated): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let inputs: Vec<f32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Enter a valid number"))
        .collect();

    if inputs.len() != 2 {
        println!("Please enter exactly two numbers.");
        return;
    }

    let (length, breadth) = (inputs[0], inputs[1]);
    let r1 = Rectangle::new(length, breadth);
    let r2 = Rectangle::new(1.0, 2.0);
    let r3 = Rectangle::new(21.0, 22.0);

    println!(
        "Area of Rectangle({} x {}) is {}",
        r1.length,
        r1.breadth,
        r1.area()
    );

    let can_hold_r2 = r1.can_hold(&r2);
    let can_hold_r3 = r1.can_hold(&r3);

    match (can_hold_r2, can_hold_r3) {
        (true, true) => println!("Input rectangle can hold both rectangles."),
        (true, false) => println!(
            "Input rectangle can hold rectangle({} x {}).",
            r2.length, r2.breadth
        ),
        (false, true) => println!(
            "Input rectangle can hold rectangle({} x {}).",
            r3.length, r3.breadth
        ),
        (false, false) => println!("Input rectangle can't hold either rectangle."),
    }
}
