#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rec1 is {}", rec1);

    println!("The are of the rectangle is {} square pixels.", area(&rec1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
