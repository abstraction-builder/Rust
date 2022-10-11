struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("amalania@qarva.com"),
        username: String::from("abstraction-builder"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("amikomalania@gmail.com");

    println!("{}", user1.email);

}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}