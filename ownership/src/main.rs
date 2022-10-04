fn main() {

    {
        let _s = "hello";
    }

    let mut my_str = String::from("hiiiiiii");

    my_str.push_str(", there");

    println!("{}", my_str);
}
