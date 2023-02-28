use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 15);
    scores.insert(String::from("Red"), 213);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} {value}");
    }
}
