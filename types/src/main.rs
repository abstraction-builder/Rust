fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x = {x} y = {y} z = {z}");

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("x = {first} y = {second} z = {third}");
}
