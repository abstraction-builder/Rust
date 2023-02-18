fn main() {

    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    println!("x = {x}");

    greet();

    let rolled_dice = fair_dice_roll();
    println!("rolled_dice={rolled_dice}");

    let _y = "out";
    {
        let y = "in";
        println!("{y}");
    }

    /**
    * Blocks are also expressions, which mean they evaluate to a value
    */
    let _x = 42; // this is equaivalent to
    let _x = { 42 }; // this

    let _z = {
        let x = 1; // first statement
        let y = 2; // second statement
        x + y      // this is the *tail* - what the block will evaluate to
    };

}


fn greet() {
    println!("Hi there");
}

fn fair_dice_roll() -> i32 {
    4
}

/* Same as above */
fn fair_dice_roll2() -> i32 {
    return 4;
}

/* If conditionals are also expressions */
fn fair_dice_roll3(feeling_lucky: bool) -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}

/* A match is also an expression */
fn fair_dice_roll4(feeling_lucky: bool) -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}