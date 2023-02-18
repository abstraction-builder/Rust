struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn main() {
    let minus_two = Number {
        odd: false,
        value: -2,
    };

    println!(
        "Is {} positive? {}",
        minus_two.value,
        minus_two.is_strictly_positive()
    );
}
