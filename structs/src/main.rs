struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: User) {
    println!(
        "User {{\n  active: {}\n  email: {}\n  username: {}\n  sign_in_count: {}\n}}",
        user.active, user.username, user.email, user.sign_in_count
    );
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("XxX_user_XxX"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("something@else.com");
    print_user(user1);
    let user2 = build_user("aaa@bbb.ccc".to_string(), "ddd".to_string());
    // print_user(user2); // Cannot compile because stuff from user2 was move to user3
    let user3 = User {
        email: "xxx@yyy.zzz".to_string(),
        ..user2
    };
    print_user(user3);
}
