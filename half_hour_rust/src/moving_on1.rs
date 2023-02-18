use std::cmp::{min, max};

struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    // let least = std::cmp::min(3, 8);
    let _least = min(15, 213);

    // `Vec` is a regular struct, not a primitive type
    let v = Vec::new();

    // This is exactly the same code, but with *full* path to `Vec`
    let _v1 = std::vec::Vec::new();

    // Structs can be initialized usint struct literals
    let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { x: 4.0, y: 7.0 };
}