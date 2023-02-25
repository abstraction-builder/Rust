fn main() {
    let vec = vec![1, 2, 3];

    let num = vec.get(4);
    match num {
        Some(num) => println!("Got {num}"),
        None => println!("Index out of range"),
    }

}
