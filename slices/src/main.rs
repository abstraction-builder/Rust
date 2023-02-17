fn main() {
    let s = String::from("foo bar");

    let _index = first_word(&s);

    let word = first_word_with_slice(&s);

    println!("{}", word);
}

fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    string.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}