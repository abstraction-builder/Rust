fn main()
{
    let x: (i32, f64, u8) = (15213, 3.14, 1);

    let icp = x.0;
    
    let pi = x.1;

    let one = x.2;

    println!("{icp} {pi} {one}");
}
