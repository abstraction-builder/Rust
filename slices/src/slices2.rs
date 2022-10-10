fn main()
{
    let my_str = String::from("hello world");
    let first_word = first_word(&my_str);

    println!("{first_word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; 
        }
    }

    &s[..]
}
