#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("foo")),
        SpreadsheetCell::Float(15.213),
    ];

    for elem in row {
        println!("{:?elem}");
    }
}
