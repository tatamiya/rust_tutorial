struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // no need to write `email: email`
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("username: {}", user1.username);

    let mut mut_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    mut_user.email = String::from("anotheremail@example.com");

    let built_email = String::from("hoge@example.com");
    let built_username = String::from("hoge");
    let built_user = build_user(built_email, built_username);
    println!(
        "username: {}, email: {}",
        built_user.username, built_user.email
    );

    // Struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("username: {}, is active?: {}", user2.username, user2.active);

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}
