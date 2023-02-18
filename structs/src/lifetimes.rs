struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: i32,
}

fn main() {
    let user1 = User {
        active: true,
        username: "abstraction-builder",
        email: "amalania@qarva.com",
        sign_in_count: 1,
    };
}