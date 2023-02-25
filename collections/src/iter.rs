fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for elem in &v {
        println!("{elem}");
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for elem in &mut v {
        *elem += 50;
        println!("{elem}");
    }
}
