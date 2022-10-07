fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

}

fn first_word(s: &string) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerat() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}
