use std::io;
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
}
fn main() {
    print!("enter length & breadth (space-separated) : ");
    io::Write::flush(&mut io::stdout()).expect("Flush Failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let inputs: Vec<f32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("enter valid number"))
        .collect();
    if inputs.len() != 2 {
        println!("Please enter exactly two numbers.");
        return;
    }
    let (l, b) = (inputs[0], inputs[1]);
    let r1 = Rectangle::new(l, b);
    println!(
        "Area of Rectangle({} x {}) is {}",
        r1.length,
        r1.breadth,
        r1.area()
    );
}
