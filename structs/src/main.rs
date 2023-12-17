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

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("XxX_user_XxX"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("something@else.com");
    let user2 = build_user("aaa@bbb.ccc".to_string(), "ddd".to_string());
}
