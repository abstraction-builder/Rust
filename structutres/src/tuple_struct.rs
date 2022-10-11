struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main()
{
    let black = Point(0, 0, 0);
    let Point(x, y, z) = black;
    println!("{} {} {}", x, y, z);
    let origin = Point(0, 0, 0);
}