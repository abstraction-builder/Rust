struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("abstraction-builder"),
        email: String::from("amikomalania@gmail.com"),
        sign_in_count: 1,
    };

    println!("Information about the user {}",user1.username);

    println!("He's email is {}", user1.email);
    println!("He's active right now {}", user1.active);
    println!("He's signed in {} times", user1.sign_in_count);

    let _user2 = build_user("amalania@qarva.com".to_string(), "amalania".to_string());

    /* Struct update syntax */
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("amiran.malania@iliauni.edu.ge"),
        sign_in_count: user1.sign_in_count,
    };

    /* Struct update syntax more compact form */
    let user4 = User {
        active: false,
        ..user3
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}