use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 10);

    scores.entry(String::from("Blue")).or_insert(15);
    scores.entry(String::from("Red")).or_insert(213);

    println!("{:?}", scores);
}
