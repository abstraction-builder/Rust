fn main()
{
    let mut s = String::from("foo");
    println!("{}", s);

    s.push_str("bar");
    println!("{}", s);
}
