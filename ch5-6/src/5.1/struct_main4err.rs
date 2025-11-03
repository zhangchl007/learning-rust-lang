struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "user1@example.com",
        username: "user1",
        sign_in_count: 1,
        active: true,
    };
    let user2 = User {
        email: "user2@example.com",
        username: "user2",
        sign_in_count: 2,
        active: false,
    };
    println!("User1: {}, {}, {}, {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("User2: {}, {}, {}, {}", user2.username, user2.email, user2.sign_in_count, user2.active);
}