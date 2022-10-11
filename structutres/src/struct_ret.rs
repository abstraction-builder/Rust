struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = build_user(String::from("amalania@qarva.com"), String::from("abstraction-builder"));

    println!("{ } { }", user1.email, user1.username);

    let user2 = User {
        email: String::from("amiran.malania@iliauni.edu.ge"),
        ..user1
    };

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}