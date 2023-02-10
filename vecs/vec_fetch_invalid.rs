fn main() {
    let v = vec![1, 2, 3];

    let third: Option<&i32> = v.get(3);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
