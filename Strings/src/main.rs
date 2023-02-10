fn main() {
    let my_str = String::new();
    println!("{}", my_str);

    let data = "initial contents";

    let s = data.to_string();

    // Or
    let s1 = "initial value".to_string();

    let s2 = String::from("initial contents");
}
