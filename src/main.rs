use std::io;
struct Rectangle {
    length: u32,
    breadth: u32,
}
impl Rectangle {
    fn new(length: u32, breadth: u32) -> Rectangle {
        Rectangle { length, breadth }
    }
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
}
fn main() {
    print!("enter length & breadth (space-separated) : ");
    io::Write::flush(&mut io::stdout()).expect("Flush Failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let inputs: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("enter valid number"))
        .collect();
    let (l, b) = (inputs[0], inputs[1]);
    let r1 = Rectangle::new(l, b);
    println!(
        "Area of Rectangle({} x {}) is {}",
        r1.length,
        r1.breadth,
        r1.area()
    );
}
