#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The rea of the rectangle with height of {} and width of {} is {} square pixels.",
        rec1.width(),
        rec1.height(),
        rec1.area()
    );

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rec3));
}
