#![allow(unused_variables)]
#[allow(dead_code)]

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);

fn main() {
    let evan = User {
        email: String::from("evan@evanrelf.com"),
        username: String::from("evanrelf"),
        active: true,
        sign_in_count: 1,
    };

    let red = Color(255, 0, 0);
}
